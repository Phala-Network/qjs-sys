use crate::attrs::trim_rust_raw;

use super::*;
use proc_macro2::Span;
use quote::format_ident;
use template_quote::quote_spanned;

impl ToTokens for Mod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_js = &self.js_crate;
        let classes = self.classes.values();
        tokens.extend(quote! {
            mod qjsbind_generated {
                #![allow(non_snake_case)]

                use super::*;
                use #crate_js as crate_js;

                #(#classes)*
            }
        });
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let rs_name = &self.name;
        let class_name_str = match &self.attrs.js_name {
            Some(js_name) => js_name.value(),
            None => rs_name.to_string(),
        };
        let mut properties = vec![];
        let constructor_var = syn::Ident::new("constructor", Span::call_site());
        let proto_var = syn::Ident::new("proto", Span::call_site());
        struct Property {
            span: Span,
            is_static: bool,
            js_name: String,
            getter: Option<(Ident, Ident)>,
            setter: Option<(Ident, Ident)>,
        }
        let mut methods = vec![];
        for field in self.fields.iter() {
            let Some(prop) = &field.qjs_property else {
                continue;
            };
            let prop_js_name = prop.js_name_str(self);
            match (prop.attrs.getter.clone(), prop.attrs.setter.clone()) {
                (Some(getter), Some(setter)) => {
                    let getter_fn_name = prop.getter_fn_name(self);
                    let setter_fn_name = prop.setter_fn_name(self);
                    properties.push(Property {
                        span: field.span(),
                        is_static: false,
                        js_name: prop_js_name,
                        getter: Some((getter, getter_fn_name)),
                        setter: Some((setter, setter_fn_name)),
                    });
                }
                (Some(getter), None) => {
                    let getter_fn_name = prop.getter_fn_name(self);
                    properties.push(Property {
                        span: field.span(),
                        is_static: false,
                        js_name: prop_js_name,
                        getter: Some((getter, getter_fn_name)),
                        setter: None,
                    });
                }
                (None, Some(setter)) => {
                    let setter_fn_name = prop.setter_fn_name(self);
                    properties.push(Property {
                        span: field.span(),
                        is_static: false,
                        js_name: prop_js_name,
                        getter: None,
                        setter: Some((setter, setter_fn_name)),
                    });
                }
                (None, None) => {}
            }
        }

        for method in self.methods.iter() {
            let js_name = method.js_name_str(self);
            let fn_name = method.impl_fn_name(self);
            let marker_token = method.attrs.marker_token.clone();
            match method.attrs.fn_type.clone() {
                MethodType::Getter => {
                    if method.is_static {
                        properties.push(Property {
                            span: method.name.span(),
                            is_static: true,
                            js_name,
                            getter: Some((marker_token, fn_name)),
                            setter: None,
                        });
                        continue;
                    }
                    if let Some(Property { getter, .. }) =
                        properties.iter_mut().find(|p| p.js_name == js_name)
                    {
                        *getter = Some((marker_token, fn_name));
                    } else {
                        properties.push(Property {
                            span: method.name.span(),
                            is_static: false,
                            js_name,
                            getter: Some((marker_token, fn_name)),
                            setter: None,
                        });
                    }
                }
                MethodType::Setter => {
                    if method.is_static {
                        properties.push(Property {
                            span: method.name.span(),
                            is_static: true,
                            js_name,
                            getter: None,
                            setter: Some((marker_token, fn_name)),
                        });
                        continue;
                    }
                    if let Some(Property { setter, .. }) =
                        properties.iter_mut().find(|p| p.js_name == js_name)
                    {
                        *setter = Some((marker_token, fn_name));
                    } else {
                        properties.push(Property {
                            span: method.name.span(),
                            is_static: false,
                            js_name,
                            getter: None,
                            setter: Some((marker_token, fn_name)),
                        });
                    }
                }
                MethodType::Method => {
                    let target = if method.is_static {
                        constructor_var.clone()
                    } else {
                        proto_var.clone()
                    };
                    methods.push(quote_spanned! { marker_token.span() =>
                        #target.define_property_fn(#js_name, #fn_name)?;
                    });
                }
                MethodType::Constructor => {
                    // This should never be called
                    continue;
                }
            }
        }

        let properties = properties.iter().map(
            |Property {
                 span,
                 is_static,
                 js_name,
                 getter,
                 setter,
             }| {
                let getter_tokens = getter
                    .as_ref()
                    .map(|(marker, ident)| quote_spanned! { marker.span() => Some(#ident) })
                    .unwrap_or(quote! { None });
                let setter_tokens = match setter.as_ref() {
                    Some((marker, ident)) => quote_spanned! { marker.span() => Some(#ident) },
                    None => {
                        if let Some(getter) = getter {
                            let error_msg = format!("property `{js_name}` is read-only");
                            quote_spanned! { getter.0.span() =>
                                {
                                    #[crate_js::host_call]
                                    fn _ro_setter(_value: crate_js::Value) -> crate_js::Result<()> {
                                        Err(crate_js::Error::msg(#error_msg))
                                    }
                                    Some(_ro_setter)
                                }
                            }
                        } else {
                            quote! { None }
                        }
                    }
                };
                let target = if *is_static {
                    constructor_var.clone()
                } else {
                    proto_var.clone()
                };
                quote_spanned! { span.clone() =>
                    #target.define_property_getset(#js_name, #getter_tokens, #setter_tokens)?;
                }
            },
        );

        tokens.extend(quote_spanned! { rs_name.span() =>
            impl crate_js::Named for #rs_name {
                const CLASS_NAME: &'static str = #class_name_str;
            }
            impl crate_js::NativeClass for #rs_name {
                fn constructor_object(ctx: &crate_js::Context) -> crate_js::Result<crate_js::Value> {
                    ctx.get_qjsbind_object(std::any::type_name::<#rs_name>(), || {
                        let #constructor_var = ctx.new_function(#class_name_str, #{self.constructor_cfn()}, 0, crate_js::c::JS_CFUNC_constructor);
                        let #proto_var = ctx.new_object(#class_name_str);
                        #(#properties)*
                        #(#methods)*
                        #constructor_var.set_property("prototype", &#proto_var)?;
                        Ok(#constructor_var)
                    })
                }
            }
        });
        for field in &self.fields {
            if let Some(prop) = &field.qjs_property {
                prop.to_tokens(tokens, self);
            }
        }
        for method in &self.methods {
            method.to_tokens(tokens, self);
        }

        let class_name = &self.name;
        if let Some(c) = &self.constructor {
            let args = c.args.args_defs();
            let args_idents = c.args.args_idents();
            tokens.extend(quote_spanned! { c.attrs.marker_token.span() =>
                #[crate_js::host_call(with_context)]
                fn #{self.constructor_cfn()}(
                    ctx: crate_js::Context,
                    _this_value: crate_js::Value,
                    #(#args),*
                ) -> crate_js::Result<crate_js::Native<#class_name>> {
                    #[allow(unused_variables)]
                    let ctx = ctx;
                    use crate_js::IntoNativeObject;
                    #class_name::#{&c.name}(#(#args_idents),*).into_native_object(&ctx)
                }
            });
        } else {
            let not_implemented = format!("{class_name} constructor not implemented");
            tokens.extend(quote_spanned! { class_name.span() =>
                #[crate_js::host_call(with_context)]
                fn #{self.constructor_cfn()}(
                    _ctx: crate_js::Context,
                    _this_value: crate_js::Value,
                ) -> crate_js::Result<crate_js::Native<#class_name>> {
                    Err(crate_js::Error::msg(#not_implemented))
                }
            });
        }
    }
}

impl Class {
    fn rename_field(&self, name: &Ident) -> Ident {
        let name = trim_rust_raw(name.clone());
        if let Some(rename_all) = &self.attrs.rename_all {
            rename_all.rename(&name)
        } else {
            name
        }
    }
    fn constructor_cfn(&self) -> Ident {
        format_ident!("qjsbind_{}_constructor", self.name)
    }
}

impl DerivedProperty {
    fn js_name_str(&self, class: &Class) -> String {
        if let Some(js_name) = &self.attrs.js_name {
            js_name.value()
        } else {
            class.rename_field(&self.name).to_string()
        }
    }

    fn getter_fn_name(&self, class: &Class) -> Ident {
        format_ident!(
            "qjsbind_instance_getter__{}_{}",
            class.name,
            self.js_name_str(class)
        )
    }
    fn setter_fn_name(&self, class: &Class) -> Ident {
        format_ident!(
            "qjsbind_instance_setter__{}_{}",
            class.name,
            self.js_name_str(class)
        )
    }

    fn to_tokens(&self, tokens: &mut TokenStream, class: &Class) {
        if let Some(getter) = &self.attrs.getter {
            let getter_fn = self.getter_fn_name(class);
            tokens.extend(quote_spanned! { getter.span() =>
                #[crate_js::host_call(with_context)]
                fn #getter_fn(_ctx: crate_js::Context, this_value: crate_js::Native<#{&class.name}>) -> #{&self.ty} {
                    this_value.borrow().#{&self.name}.clone()
                }
            });
        }

        if let Some(setter) = &self.attrs.setter {
            let setter_fn = self.setter_fn_name(class);
            tokens.extend(quote_spanned! { setter.span() =>
                #[crate_js::host_call(with_context)]
                fn #setter_fn(_ctx: crate_js::Context, this_value: crate_js::Native<#{&class.name}>, value: #{&self.ty}) {
                    this_value.borrow_mut().#{&self.name} = value;
                }
            });
        }
    }
}

impl Args {
    fn args_idents(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.args.iter().map(|arg| {
            if let Some(from_context) = &arg.from_context {
                quote_spanned! {
                    from_context.span() =>
                    crate_js::FromJsContext::from_js_context(&ctx)?
                }
            } else {
                quote_spanned! { arg.name.span() =>  #{&arg.name} }
            }
        })
    }

    fn args_defs(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.args.iter().flat_map(|arg| {
            if arg.from_context.is_some() {
                None
            } else {
                Some(quote_spanned! { arg.name.span() =>  #{&arg.name}: #{&arg.ty} })
            }
        })
    }
}

impl Method {
    fn js_name_str(&self, class: &Class) -> String {
        if let Some(js_name) = &self.attrs.js_name {
            js_name.value()
        } else {
            class.rename_field(&self.name).to_string()
        }
    }

    fn impl_fn_name(&self, class: &Class) -> Ident {
        let js_name_str = self.js_name_str(class);
        let class_name = &class.name;
        let static_str = if self.is_static { "static" } else { "instance" };
        match self.attrs.fn_type {
            MethodType::Getter => {
                format_ident!("qjsbind_{static_str}_getter__{class_name}_{js_name_str}")
            }
            MethodType::Setter => {
                format_ident!("qjsbind_{static_str}_setter__{class_name}_{js_name_str}")
            }
            MethodType::Method => {
                format_ident!("qjsbind_{static_str}_method__{class_name}_{js_name_str}")
            }
            MethodType::Constructor => {
                // This should never be called
                format_ident!("qjsbind_{class_name}_constructor")
            }
        }
    }

    fn to_tokens(&self, tokens: &mut TokenStream, class: &Class) {
        let name = &self.name;

        let fn_name = self.impl_fn_name(class);
        let class_name = &class.name;
        let args = self.args.args_defs();
        let args_idents = self.args.args_idents();

        tokens.extend(quote_spanned! { self.attrs.marker_token.span() =>
            #[crate_js::host_call(with_context)]
            fn #fn_name(ctx: crate_js::Context, this_value: crate_js::Native<#class_name>, #(#args),*) #{&self.return_ty} {
                #[allow(unused_variables)]
                let ctx = ctx;
                #(if self.is_static) {
                    let _ = this_value;
                    #class_name::
                }
                #(else if self.is_mut) {
                    this_value.borrow_mut().
                }
                #(else) {
                    this_value.borrow().
                }
                    #name(#(#args_idents),*)
            }
        });
    }
}
