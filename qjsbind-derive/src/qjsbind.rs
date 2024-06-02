//! This module contains the `qjsbind` attribute macro implementation.
//!

use proc_macro2::TokenStream;
use std::collections::BTreeMap;
use syn::parse::Parser;
use syn::{
    Attribute, Field, Ident, ImplItemFn, Item, ItemMod, ItemStruct, LitStr, Path, Receiver, Result,
    Type,
};
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
    methods: Vec<Method>,
    fields: Vec<ClassField>,
    attrs: ClassAttrs,
}

struct ClassField {
    field: Field,
    qjs_property: Option<DerivedProperty>,
}

impl ClassField {
    fn no_gc(&self) -> bool {
        self.qjs_property.as_ref().map_or(false, |p| p.attrs.no_gc)
    }
}

struct ClassAttrs {
    js_name: Option<LitStr>,
    rename_all: Option<RenameAll>,
}

struct DerivedProperty {
    name: Ident,
    ty: Type,
    attrs: FieldAttrs,
}

struct FieldAttrs {
    js_name: Option<LitStr>,
    getter: Option<Ident>,
    setter: Option<Ident>,
    no_gc: bool,
}

struct ArgSelf {
    is_ref: bool,
    is_mut: bool,
    token: Receiver,
}

#[derive(Default)]
struct Args {
    receiver: Option<ArgSelf>,
    args: Vec<Arg>,
}

struct Arg {
    name: Ident,
    ty: Type,
    from_context: Option<Ident>,
}

struct Constructor {
    name: Ident,
    args: Args,
    attrs: ConstructorAttrs,
}

struct ConstructorAttrs {
    marker_token: Ident,
}

struct Method {
    name: Ident,
    args: Args,
    return_ty: syn::ReturnType,
    is_mut: bool,
    is_static: bool,
    attrs: MethodAttrs,
}

struct MethodAttrs {
    js_name: Option<LitStr>,
    fn_type: MethodType,
    marker_token: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MethodType {
    Getter,
    Setter,
    Method,
    Constructor,
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
        syn_bail!(semi, "expected a module with content");
    };

    Ok(quote! {
        #(#attrs)*
        #vis #unsafety #mod_token #ident {
            #(#content)*
            #qjs_mod
        }
    })
}
