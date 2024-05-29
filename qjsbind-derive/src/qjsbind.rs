//! This module contains the `qjsbind` attribute macro implementation.
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
    constructor: Option<Constructor>,
    derived_properties: Vec<DerivedProperty>,
    methods: Vec<Method>,
    gc_mark_fields: Vec<Ident>,
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
    gc_mark: bool,
}

struct Constructor {
    name: Ident,
    args: Vec<(syn::Ident, syn::Type)>,
}

struct Method {
    name: Ident,
    args: Vec<(syn::Ident, syn::Type)>,
    return_ty: syn::ReturnType,
    is_mut: bool,
    attrs: MethodAttrs,
}

struct MethodAttrs {
    js_name: Option<LitStr>,
    fn_type: MethodType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MethodType {
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
