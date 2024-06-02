use proc_macro2::TokenStream;
use template_quote::quote;

use super::{
    attrs::{ContainerAttrs, FieldAttrs},
    bound::where_clause_with_bound,
    find_crate_name,
};

pub fn derive(input: &mut syn::DeriveInput, from_js: bool, into: bool) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => derive_struct(input, fields, from_js, into),
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fields),
            ..
        }) if fields.unnamed.len() == 1 => derive_newtype_struct(input, from_js, into),
        _ => panic!("only structs with named fields are supported"),
    }
}

fn derive_newtype_struct(
    input: &syn::DeriveInput,
    from_js: bool,
    into: bool,
) -> syn::Result<TokenStream> {
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
        let (trait_name, fn_name, self_arg);
        if into {
            trait_name = quote!(IntoJsValue);
            fn_name = quote!(into_js_value);
            self_arg = quote!(self);
        } else {
            trait_name = quote!(ToJsValue);
            fn_name = quote!(to_js_value);
            self_arg = quote!(&self);
        }
        let bound = syn::parse_quote!(#crate_qjsbind::#trait_name);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, #trait_name, Result};
                impl #impl_generics #trait_name for #ident #ty_generics #bounded_where_clause {
                    fn #fn_name(#self_arg, ctx: &#crate_qjsbind::Context) -> Result<Value> {
                        let value = self.0.#fn_name(ctx);
                        value.set_name(#{ident.to_string()});
                        value
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
    into: bool,
) -> syn::Result<TokenStream> {
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();

    let crate_qjsbind = find_crate_name("qjsbind")?;
    let container_attrs = ContainerAttrs::of(input)?;
    let ident = container_attrs.ident();
    let attrs = fields
        .named
        .iter()
        .map(FieldAttrs::of)
        .collect::<syn::Result<Vec<_>>>()?;

    if from_js {
        let bound = syn::parse_quote!(#crate_qjsbind::FromJsValue);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);

        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, FromJsValue, Result, Error, alloc, ErrorContext as _};
                impl #impl_generics FromJsValue for #ident #ty_generics #bounded_where_clause {
                    fn from_js_value(val: Value) -> Result<Self> {
                        #(if container_attrs.allow_default()) {
                            if val.is_null_or_undefined() {
                                return Ok(<Self as Default>::default());
                            }
                        }
                        Ok(Self {
                            #(for field in &attrs) {
                                #{&field.field().ident}: {
                                    let field_value = val.get_property(#{field.js_name(&container_attrs)})?;
                                    #{
                                        match field.default_fn() {
                                            Some(f) => {
                                                quote! {
                                                    if field_value.is_null_or_undefined() {
                                                        #f()
                                                    } else {
                                                        let field_name = #{&field.field().ident.as_ref().map(|f| f.to_string()).unwrap_or_default()};
                                                        #{field.decoder_fn(&crate_qjsbind)}(field_value)
                                                            .context(format!("failed to decode field {field_name}"))?
                                                    }
                                                }
                                            }
                                            None => quote! {
                                                #{field.decoder_fn(&crate_qjsbind)}(field_value)?
                                            },
                                        }
                                    }
                                },
                            }
                        })
                    }
                }
            };
        })
    } else {
        let (trait_name, fn_name, self_arg);
        if into {
            trait_name = quote!(IntoJsValue);
            fn_name = quote!(into_js_value);
            self_arg = quote!(self);
        } else {
            trait_name = quote!(ToJsValue);
            fn_name = quote!(to_js_value);
            self_arg = quote!(&self);
        }
        let bound = syn::parse_quote!(#crate_qjsbind::#trait_name);
        let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
        Ok(quote! {
            const _: () = {
                use #crate_qjsbind::{c, Value, #trait_name, Result};
                impl #impl_generics #trait_name for #ident #ty_generics #bounded_where_clause {
                    fn #fn_name(#self_arg, ctx: &#crate_qjsbind::Context) -> Result<Value> {
                        let obj = ctx.new_object(#{ident.to_string()});
                        #(for field in &attrs) {
                            #(if field.as_bytes() || field.bytes_or_hex()) {
                                let field_value = #crate_qjsbind::encode_as_bytes(ctx, &self.#{&field.field().ident});
                            }
                            #(else) {
                                let field_value = self.#{&field.field().ident}.#fn_name(ctx)?;
                            }
                            obj.set_property(#{field.js_name(&container_attrs)}, &field_value)?;
                        }
                        Ok(obj)
                    }
                }
            };
        })
    }
}
