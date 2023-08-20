use proc_macro2::{Span, TokenStream};
use std::collections::HashSet;
use template_quote::quote;

use super::{
    attrs::{ContainerAttrs, FieldAttrs, TypeDefault},
    bound::{where_clause_with_bound, with_lifetime_bound},
    find_crate_name,
};

pub fn derive(input: &mut syn::DeriveInput, from_js: bool) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => derive_struct(input, fields, from_js),
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fields),
            ..
        }) if fields.unnamed.len() == 1 => derive_newtype_struct(input, from_js),
        _ => panic!("only structs with named fields are supported"),
    }
}

fn derive_newtype_struct(input: &syn::DeriveInput, from_js: bool) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();

    let crate_qjsbind = find_crate_name("qjsbind")?;
    if from_js {
        let bound = syn::parse_quote!(#crate_qjsbind::FromJsValue);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, FromJsValue, Result};
                impl #impl_generics FromJsValue for #ident #ty_generics #bounded_where_clause {
                    fn from_js_value(js_value: Value) -> Result<Self> {
                        Ok(Self(FromJsValue::from_js_value(js_value)?))
                    }
                }
            };
        })
    } else {
        let bound = syn::parse_quote!(#crate_qjsbind::ToJsValue);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, ToJsValue, Result};
                impl #impl_generics ToJsValue for #ident #ty_generics #bounded_where_clause {
                    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
                        self.0.to_js_value(ctx)
                    }
                }
            };
        })
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
    from_js: bool,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();

    let crate_qjsbind = find_crate_name("qjsbind")?;
    let container_attrs = ContainerAttrs::of(input)?;
    let type_name = container_attrs.container_name();
    let attrs = fields
        .named
        .iter()
        .map(FieldAttrs::of)
        .collect::<syn::Result<Vec<_>>>()?;
    let fieldname = attrs.iter().map(|x| &x.field().ident).collect::<Vec<_>>();

    let wrapper_generics = with_lifetime_bound(&input.generics, "'__a");
    let (wrapper_impl_generics, wrapper_ty_generics, _) = wrapper_generics.split_for_impl();
    let bound = syn::parse_quote!(::deser::Serialize);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);

    if from_js {
        let bound = syn::parse_quote!(#crate_qjsbind::FromJsValue);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
        let values_or_defaults = attrs
            .iter()
            .map(|attrs| match attrs.default() {
                Some(TypeDefault::Implicit) => {
                    quote! { take().unwrap_or_else(Default::default) }
                }
                Some(TypeDefault::Explicit(path)) => {
                    quote! { take().unwrap_or_else(#path) }
                }
                None => quote!(take()),
            })
            .collect::<Vec<_>>();

        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, FromJsValue, Result};
                impl #impl_generics FromJsValue for #ident #ty_generics #bounded_where_clause {
                    fn from_js_value(js_value: Value) -> Result<Self> {
                        todo!()
                    }
                }
            };
        })
    } else {
        let bound = syn::parse_quote!(#crate_qjsbind::ToJsValue);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, ToJsValue, Result};
                impl #impl_generics ToJsValue for #ident #ty_generics #bounded_where_clause {
                    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
                        todo!()
                    }
                }
            };
        })
    }
}
