use proc_macro2::TokenStream;
use quote::format_ident;
use syn::spanned::Spanned;
use template_quote::{quote, quote_spanned};

use super::{bound::where_clause_with_bound, find_crate_name};

pub fn derive(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(s) => derive_struct(input, s),
        syn::Data::Enum(e) => derive_enum(input, e),
        _ => panic!("only structs are supported"),
    }
}

struct FieldAttrs {
    no_gc: bool,
}

impl FieldAttrs {
    pub fn parse(field: &syn::Field) -> syn::Result<FieldAttrs> {
        let mut rv = FieldAttrs { no_gc: false };

        for attr in field.attrs.iter() {
            if !attr.path().is_ident("gc") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    rv.no_gc = true;
                } else {
                    syn_bail!(meta.path, "unsupported attribute");
                }
                Ok(())
            })?;
        }
        Ok(rv)
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    item_struct: &syn::DataStruct,
) -> syn::Result<TokenStream> {
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();

    let crate_qjsbind = find_crate_name("qjsbind")?;
    let mut fields = vec![];
    for field in item_struct.fields.iter() {
        fields.push((field, FieldAttrs::parse(field)?));
    }

    let bound = syn::parse_quote!(#crate_qjsbind::GcMark);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    Ok(quote! {
        const _: () = {
            use #crate_qjsbind::{c, Value, GcMark};
            impl #impl_generics GcMark for #{&input.ident} #ty_generics #bounded_where_clause {
                fn gc_mark(&self, rt: *mut c::JSRuntime, mrk: c::JS_MarkFunc) {
                    #[allow(unused_variables)]
                    let (rt, mrk) = (rt, mrk);
                    #(for (i, (field, attrs)) in fields.iter().enumerate()) {
                        #(if !attrs.no_gc) {
                            #{
                                if let Some(ident) = &field.ident {
                                    quote_spanned!{ field.span() => GcMark::gc_mark(&self.#ident, rt, mrk); }
                                } else {
                                    let ind = syn::Index::from(i);
                                    quote_spanned! { field.span() => GcMark::gc_mark(&self.#ind, rt, mrk); }
                                }
                            }
                        }
                    }
                }
            }
        };
    })
}

fn derive_enum(input: &syn::DeriveInput, item_enum: &syn::DataEnum) -> syn::Result<TokenStream> {
    let crate_qjsbind = find_crate_name("qjsbind")?;
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();
    let bound = syn::parse_quote!(#crate_qjsbind::GcMark);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    fn match_case(variant: &syn::Variant) -> syn::Result<TokenStream> {
        let name = &variant.ident;
        let tokens = match &variant.fields {
            syn::Fields::Unit => {
                quote_spanned! { variant.span() => #name => {} }
            }
            syn::Fields::Named(fields) => {
                let matched_fields = fields
                    .named
                    .iter()
                    .map(|f| {
                        let attr = FieldAttrs::parse(f)?;
                        let name = f.ident.as_ref().ok_or(syn::Error::new(
                            f.span(),
                            "named fields must have an identifier",
                        ))?;
                        Ok((name, attr))
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                quote_spanned!(variant.span() => #name { #(for (f, _) in &matched_fields), { #f } } => {
                    #(for (field, attr) in &matched_fields) {
                        #(if !attr.no_gc ) {
                            GcMark::gc_mark(#field, rt, mrk);
                        }
                        #(else) {
                            _ = #field;
                        }
                    }
                })
            }
            syn::Fields::Unnamed(fields) => {
                let matched_fields = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let attr = FieldAttrs::parse(f)?;
                        let name = format_ident!("_{}", i);
                        Ok((name, attr))
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                quote_spanned!(variant.span() => #name ( #(for (f, _) in &matched_fields), { #f }  ) => {
                    #(for (field, attr) in &matched_fields) {
                        #(if !attr.no_gc ) {
                            GcMark::gc_mark(#field, rt, mrk);
                        }
                    }
                })
            }
        };
        Ok(tokens)
    }
    Ok(quote! {
        const _: () = {
            use #crate_qjsbind::{c, Value, GcMark};
            impl #impl_generics GcMark for #{&input.ident} #ty_generics #bounded_where_clause {
                fn gc_mark(&self, rt: *mut c::JSRuntime, mrk: c::JS_MarkFunc) {
                    #[allow(unused_variables)]
                    let (rt, mrk) = (rt, mrk);
                    match self {
                        #(for variant in item_enum.variants.iter()) {
                            #{&input.ident}::#{match_case(variant)?}
                        }
                    }
                }
            }
        };
    })
}

#[test]
fn show_tokens() {
    let input: syn::DeriveInput = syn::parse_quote! {
        struct Test(#[gc(skip)] i32, String, Value, u32);
    };
    let generated = derive(&input).unwrap();
    insta::assert_snapshot!(rustfmt_snippet::rustfmt(&generated.to_string()).unwrap());

    let input: syn::DeriveInput = syn::parse_quote! {
        enum Test {
            A,
            B(String),
            C(u32, #[gc(skip)] i32, String),
            D { a: i32, b: String, #[gc(skip)] c: u32 },
        }
    };
    let generated = derive(&input).unwrap();
    insta::assert_snapshot!(rustfmt_snippet::rustfmt(&generated.to_string()).unwrap());
}
