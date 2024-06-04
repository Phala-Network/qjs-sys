use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use template_quote::{quote, quote_spanned};

use super::{bound::where_clause_with_bound, find_crate_name};

pub fn derive(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(s) => derive_struct(input, s),
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
                fn gc_mark(&mut self, rt: *mut c::JSRuntime, mrk: c::JS_MarkFunc) {
                    #[allow(unused_variables)]
                    let (rt, mrk) = (rt, mrk);
                    #(for (i, (field, attrs)) in fields.iter().enumerate()) {
                        #(if !attrs.no_gc) {
                            #{
                                if let Some(ident) = &field.ident {
                                    quote_spanned!{ field.span() => GcMark::gc_mark(&mut self.#ident, rt, mrk); }
                                } else {
                                    let ind = syn::Index::from(i);
                                    quote_spanned! { field.span() => GcMark::gc_mark(&mut self.#ind, rt, mrk); }
                                }
                            }
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
        struct Test(#[qjs(no_gc)] i32, String, Value, u32);
    };
    let generated = derive(&input).unwrap();
    insta::assert_snapshot!(rustfmt_snippet::rustfmt(&generated.to_string()).unwrap());
}
