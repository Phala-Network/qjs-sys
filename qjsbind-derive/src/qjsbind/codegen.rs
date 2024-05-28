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
                use super::*;
                use #crate_js as crate_js;

                use crate_js::Native;
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
                FnType::Getter => {
                    if let Some((_, setter_fn_name, _)) =
                        properties.iter_mut().find(|(name, _, _)| name == &js_name)
                    {
                        *setter_fn_name = Some(fn_name);
                    } else {
                        properties.push((js_name, Some(fn_name), None));
                    }
                }
                FnType::Setter => {
                    if let Some((_, getter_fn_name, _)) =
                        properties.iter_mut().find(|(name, _, _)| name == &js_name)
                    {
                        *getter_fn_name = Some(fn_name);
                    } else {
                        properties.push((js_name, None, Some(fn_name)));
                    }
                }
                FnType::Method => {
                    methods.push(quote! {
                        class.add_method(#js_name, #fn_name);
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
                class.add_property(#name, #getter, #setter);
            }
        });

        tokens.extend(quote! {
            impl crate_js::NativeClass for #name {
                const CLASS_NAME: &'static str = #class_name_str;
                fn class_object(ctx: crate_js::Context) -> crate_js::Value {
                    let obj = ctx.get_object(#full_class_name_str);
                    if !obj.is_undefined() {
                        return obj;
                    }
                    let class = ctx.new_class(#class_name_str);
                    #(#properties)*
                    #(#methods)*
                    ctx.set_object(#full_class_name_str, class.clone());
                    class
                }
            }
        });
        for prop in &self.derived_properties {
            prop.to_tokens(tokens, self);
        }
        for method in &self.methods {
            method.to_tokens(tokens, self);
        }
    }
}

impl Class {
    fn rename_field(&self, name: &Ident) -> Ident {
        if let Some(rename_all) = &self.attrs.rename_all {
            rename_all.rename(&name)
        } else {
            name.clone()
        }
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
                fn #getter_fn(_ctx: crate_js::Context, this_value: Native<#{&class.name}>) -> #{&self.ty} {
                    this_value.#{&self.name}.clone()
                }
            });
        }

        if self.attrs.is_setter {
            let setter_fn = self.setter_fn_name(class);
            tokens.extend(quote! {
                #[crate_js::host_call(with_context)]
                fn #setter_fn(_ctx: crate_js::Context, mut this_value: Native<#{&class.name}>, value: #{&self.ty}) {
                    this_value.#{&self.name} = value;
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
            FnType::Getter => format_ident!("{class_name}_getter__{js_name_str}"),
            FnType::Setter => format_ident!("{class_name}_setter__{js_name_str}"),
            FnType::Method => format_ident!("{class_name}_method__{js_name_str}"),
        }
    }

    fn to_tokens(&self, tokens: &mut TokenStream, class: &Class) {
        let name = &self.name;

        let fn_name = self.impl_fn_name(class);
        let class_name = format_ident!("{}", class.name);
        let args = self.args.iter().map(|(name, ty)| {
            quote! { #name: #ty }
        });
        let args_idents = self.args.iter().map(|(name, _ty)| {
            quote! { #name }
        });
        match self.attrs.fn_type {
            FnType::Getter => {
                tokens.extend(quote! {
                    #[crate_js::host_call(with_context)]
                    fn #fn_name(_ctx: crate_js::Context, this_value: Native<#class_name>, #(#args),*) #{&self.return_ty} {
                        this_value.#name()
                    }
                });
            }
            FnType::Setter => {
                tokens.extend(quote! {
                    #[crate_js::host_call(with_context)]
                    fn #fn_name(_ctx: crate_js::Context, mut this_value: Native<#class_name>, #(#args),*) #{&self.return_ty} {
                        this_value.#name(#(#args_idents),*)
                    }
                });
            }
            FnType::Method => {
                tokens.extend(quote! {
                    #[crate_js::host_call(with_context)]
                    fn #fn_name(_ctx: crate_js::Context, this_value: Native<#class_name>, #(#args),*) #{&self.return_ty} {
                        this_value.#name(#(#args_idents),*)
                    }
                });
            }
        }
    }
}
