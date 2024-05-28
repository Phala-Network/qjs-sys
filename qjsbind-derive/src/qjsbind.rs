//! This module contains the `qjsbind` attribute macro implementation.
//!
//! # Example
//! ```
//! #[qjsbind]
//! mod bindings {
//!     #[qjs(class(js_name="Request"))]
//!     struct JsRequest {
//!         #[qjs(getter)]
//!         method: String,
//!         #[qjs(getter, setter)]
//!         base_url: String,
//!         #[qjs(getter)]
//!         path: String,
//!     }
//!
//!     impl JsRequest {
//!         #[qjs(setter, js_name="url")]
//!         fn set_url(&mut self, url: String) -> Result<()> {
//!             let (base_url, path) = parse_url(&url)?;
//!             self.base_url = base_url;
//!             self.path = path;
//!         }
//!         #[qjs(method, js_name="url")]
//!         fn url(&self) -> String {
//!             format!("{}{}", self.base_url, self.path)
//!         }
//!     }
//! }
//! ```
//! Will generate the following code:
//! ```
//! mod generated {
//!     use super::*;
//!     use ::qjsbind::Native;
//!     
//!     #[::qjsbind::host_call(with_context)]
//!     fn JsRequest_getter__method(_ctx: ::qjsbind::Context, this_value: Native<JsRequest>) -> String {
//!         this_value.method.clone()
//!     }
//!
//!     #[::qjsbind::host_call(with_context)]
//!     fn JsRequest_getter__base_url(_ctx: ::qjsbind::Context, this_value: Native<JsRequest>) -> String {
//!         this_value.base_url.clone()
//!     }
//!
//!     #[::qjsbind::host_call(with_context)]
//!     fn JsRequest_getter__path(_ctx: ::qjsbind::Context, this_value: Native<JsRequest>) -> String {
//!         this_value.path.clone()
//!     }
//!
//!     #[::qjsbind::host_call(with_context)]
//!     fn JsRequest_setter__base_url(_ctx: ::qjsbind::Context, mut this_value: Native<JsRequest>, value: String) {
//!         this_value.base_url = value;
//!     }
//!
//!     #[::qjsbind::host_call(with_context)]
//!     fn JsRequest_compute_setter__url(_ctx: ::qjsbind::Context, mut this_value: Native<JsRequest>, value: String) -> Result<()> {
//!         this_value.set_url(value)
//!     }
//!
//!     #[::qjsbind::host_call(with_context)]
//!     fn JsRequest_method__url(_ctx: ::qjsbind::Context, this_value: Native<JsRequest>) -> String {
//!         this_value.url()
//!     }
//!
//!     impl JsRequest {
//!         pub fn class_object() -> js::Value {
//!             let class = js::Class::new("Request");
//!             class.add_property("method", JsRequest_getter__method, None);
//!             class.add_property("base_url", JsRequest_getter__base_url, JsRequest_setter__base_url);
//!             class.add_property("path", JsRequest_getter__path, None);
//!             class.add_method("url", JsRequest_method__url);
//!         }
//!     }
//! }
//! ```
//!

use proc_macro2::TokenStream;
use std::collections::BTreeMap;
use syn::parse::Parser;
use syn::{Attribute, Field, Ident, ImplItemFn, Item, ItemMod, ItemStruct, LitStr, Path, Result};
use template_quote::{quote, ToTokens};

use crate::attrs::RenameAll;

mod codegen;
mod parse;

struct Mod {
    js_crate: Option<Path>,
    classes: BTreeMap<String, Class>,
}

struct Class {
    name: Ident,
    derived_properties: Vec<DerivedProperty>,
    methods: Vec<Method>,
    attrs: ClassAttrs,
}

struct ClassAttrs {
    js_name: Option<LitStr>,
    rename_all: Option<RenameAll>,
}

struct DerivedProperty {
    name: Ident,
    ty: syn::Type,
    attrs: FieldAttrs,
}

struct FieldAttrs {
    js_name: Option<LitStr>,
    is_getter: bool,
    is_setter: bool,
}

struct Method {
    name: Ident,
    args: Vec<(syn::Ident, syn::Type)>,
    return_ty: syn::ReturnType,
    attrs: FnAttrs,
}

struct FnAttrs {
    js_name: Option<LitStr>,
    fn_type: FnType,
}

enum FnType {
    Getter,
    Setter,
    Method,
}

pub(crate) fn patch(config: TokenStream, input: TokenStream) -> TokenStream {
    match patch_or_err(config, input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn patch_or_err(config: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut js_crate = None;
    syn::meta::parser(|meta| {
        if meta.path.is_ident("js_crate") {
            js_crate = Some(meta.value()?.parse::<Path>()?);
        }
        Ok(())
    })
    .parse2(config)?;
    let mut the_mod: ItemMod = syn::parse2(input)?;
    let qjs_mod = Mod::from_mod(&mut the_mod, js_crate)?;
    let ItemMod {
        attrs,
        vis,
        unsafety,
        mod_token,
        ident,
        content,
        semi,
    } = the_mod;

    let Some((_brace, content)) = content else {
        return Err(syn::Error::new_spanned(
            semi,
            "expected a module with content",
        ));
    };

    Ok(quote! {
        #(#attrs)*
        #vis #unsafety #mod_token #ident {
            #(#content)*
            #qjs_mod
        }
    })
}
