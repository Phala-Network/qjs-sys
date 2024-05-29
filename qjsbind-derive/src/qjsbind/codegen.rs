use crate::attrs::trim_rust_raw;

use super::*;
use quote::format_ident;

impl ToTokens for Mod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_js = self.js_crate.clone().unwrap_or_else(|| {
            let js_crate = crate::find_crate_name("qjsbind").expect("qjsbind crate not found");
            syn::parse_quote!(:: #js_crate)
        });
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
        let name = &self.name;
        let (class_name_str, full_class_name_str) = if let Some(js_name) = &self.attrs.js_name {
            let s = js_name.value();
            let parts = s.rsplitn(2, ".").collect::<Vec<_>>();
            if parts.len() == 2 {
                (parts[0].to_string(), s)
            } else {
                (s.clone(), s)
            }
        } else {
            (name.to_string(), name.to_string())
        };
        let mut properties = vec![];
        let mut methods = vec![];
        for prop in self.derived_properties.iter() {
            let prop_js_name = prop.js_name_str(self);
            match (prop.attrs.is_getter, prop.attrs.is_setter) {
                (true, true) => {
                    let getter_fn_name = prop.getter_fn_name(self);
                    let setter_fn_name = prop.setter_fn_name(self);
                    properties.push((prop_js_name, Some(getter_fn_name), Some(setter_fn_name)));
                }
                (true, false) => {
                    let getter_fn_name = prop.getter_fn_name(self);
                    properties.push((prop_js_name, Some(getter_fn_name), None));
                }
                (false, true) => {
                    let setter_fn_name = prop.setter_fn_name(self);
                    properties.push((prop_js_name, None, Some(setter_fn_name)));
                }
                (false, false) => {}
            }
        }

        for method in self.methods.iter() {
            let js_name = method.js_name_str(self);
            let fn_name = method.impl_fn_name(self);
            match method.attrs.fn_type {
                MethodType::Getter => {
                    if let Some((_, setter_fn_name, _)) =
                        properties.iter_mut().find(|(name, _, _)| name == &js_name)
                    {
                        *setter_fn_name = Some(fn_name);
                    } else {
                        properties.push((js_name, Some(fn_name), None));
                    }
                }
                MethodType::Setter => {
                    if let Some((_, getter_fn_name, _)) =
                        properties.iter_mut().find(|(name, _, _)| name == &js_name)
                    {
                        *getter_fn_name = Some(fn_name);
                    } else {
                        properties.push((js_name, None, Some(fn_name)));
                    }
                }
                MethodType::Method => {
                    methods.push(quote! {
                        proto.define_property_fn(#js_name, #fn_name)?;
                    });
                }
            }
        }

        let properties = properties.iter().map(|(name, getter, setter)| {
            let getter = getter
                .as_ref()
                .map(|ident| quote! { Some(#ident) })
                .unwrap_or(quote! { None });
            let setter = setter
                .as_ref()
                .map(|ident| quote! { Some(#ident) })
                .unwrap_or(quote! { None });
            quote! {
                proto.define_property_getset(#name, #getter, #setter)?;
            }
        });

        tokens.extend(quote! {
            impl crate_js::GcMark for #name {}
            impl crate_js::NativeClass for #name {
                const CLASS_NAME: &'static str = #class_name_str;
                fn constructor_object(ctx: &crate_js::Context) -> crate_js::Result<crate_js::Value> {
                    let obj = ctx.lookup_object(#full_class_name_str)?;
                    if !obj.is_undefined() {
                        return Ok(obj);
                    }
                    let constructor = ctx.new_function(#class_name_str, #{self.constructor_cfn()}, 0, crate_js::c::JS_CFUNC_constructor);
                    let proto = ctx.new_object();
                    proto.set_property_atom(crate_js::c::JS_ATOM_Symbol_toStringTag, &ctx.new_string(#class_name_str))?;
                    #(#properties)*
                    #(#methods)*
                    constructor.set_property("prototype", &proto)?;
                    ctx.store_object(#full_class_name_str, constructor.clone())?;
                    Ok(constructor)
                }
            }
        });
        for prop in &self.derived_properties {
            prop.to_tokens(tokens, self);
        }
        for method in &self.methods {
            method.to_tokens(tokens, self);
        }

        let class_name = &self.name;
        if let Some(c) = &self.constructor {
            let args = c.args.iter().map(|(name, ty)| {
                quote! { #name: #ty }
            });
            let args_idents = c.args.iter().map(|(name, _ty)| {
                quote! { #name }
            });
            tokens.extend(quote! {
                #[crate_js::host_call(with_context)]
                fn #{self.constructor_cfn()}(
                    ctx: crate_js::Context,
                    _this_value: crate_js::Value,
                    #(#args),*
                ) -> crate_js::Result<crate_js::Native<#class_name>> {
                        use crate_js::IntoNativeObject;
                        #class_name::#{&c.name}(#(#args_idents),*).into_native_object(&ctx)
                }
            });
        } else {
            let not_implemented = format!("{class_name} constructor not implemented");
            tokens.extend(quote! {
                #[crate_js::host_call(with_context)]
                fn #{self.constructor_cfn()}(
                    ctx: crate_js::Context,
                    _this_value: crate_js::Value,
                ) -> crate_js::Result<crate_js::Native<#class_name>> {
                    Err(crate_js::Error::Static(#not_implemented))
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
        format_ident!("{}_constructor", self.name)
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
        format_ident!("{}_getter__{}", class.name, self.js_name_str(class))
    }
    fn setter_fn_name(&self, class: &Class) -> Ident {
        format_ident!("{}_setter__{}", class.name, self.js_name_str(class))
    }

    fn to_tokens(&self, tokens: &mut TokenStream, class: &Class) {
        if self.attrs.is_getter {
            let getter_fn = self.getter_fn_name(class);
            tokens.extend(quote! {
                #[crate_js::host_call(with_context)]
                fn #getter_fn(_ctx: crate_js::Context, this_value: crate_js::Native<#{&class.name}>) -> #{&self.ty} {
                    this_value.borrow().#{&self.name}.clone()
                }
            });
        }

        if self.attrs.is_setter {
            let setter_fn = self.setter_fn_name(class);
            tokens.extend(quote! {
                #[crate_js::host_call(with_context)]
                fn #setter_fn(_ctx: crate_js::Context, mut this_value: crate_js::Native<#{&class.name}>, value: #{&self.ty}) {
                    this_value.borrow_mut().#{&self.name} = value;
                }
            });
        }
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
        match self.attrs.fn_type {
            MethodType::Getter => format_ident!("{class_name}_getter__{js_name_str}"),
            MethodType::Setter => format_ident!("{class_name}_setter__{js_name_str}"),
            MethodType::Method => format_ident!("{class_name}_method__{js_name_str}"),
        }
    }

    fn to_tokens(&self, tokens: &mut TokenStream, class: &Class) {
        let name = &self.name;

        let fn_name = self.impl_fn_name(class);
        let class_name = &class.name;
        let args = self.args.iter().map(|(name, ty)| {
            quote! { #name: #ty }
        });
        let args_idents = self.args.iter().map(|(name, _ty)| {
            quote! { #name }
        });
        match self.attrs.fn_type {
            MethodType::Getter => {
                tokens.extend(quote! {
                    #[crate_js::host_call(with_context)]
                    fn #fn_name(_ctx: crate_js::Context, this_value: crate_js::Native<#class_name>, #(#args),*) #{&self.return_ty} {
                        this_value.borrow().#name()
                    }
                });
            }
            MethodType::Setter => {
                tokens.extend(quote! {
                    #[crate_js::host_call(with_context)]
                    fn #fn_name(_ctx: crate_js::Context, mut this_value: crate_js::Native<#class_name>, #(#args),*) #{&self.return_ty} {
                        this_value.borrow_mut().#name(#(#args_idents),*)
                    }
                });
            }
            MethodType::Method => {
                tokens.extend(quote! {
                    #[crate_js::host_call(with_context)]
                    fn #fn_name(_ctx: crate_js::Context, this_value: crate_js::Native<#class_name>, #(#args),*) #{&self.return_ty} {
                        #(if self.is_mut) {
                            this_value.borrow_mut().#name(#(#args_idents),*)
                        }
                        #(else) {
                            this_value.borrow().#name(#(#args_idents),*)
                        }
                    }
                });
            }
        }
    }
}