use proc_macro::TokenStream;
use syn::parse_macro_input;

mod attrs;
mod bound;
mod derive;
mod host_fn;
mod qjsbind;

#[proc_macro_derive(IntoJsValue, attributes(qjs))]
pub fn derive_into_js_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    derive::derive(&mut input, false, true)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ToJsValue, attributes(qjs))]
pub fn derive_to_js_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    derive::derive(&mut input, false, false)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(FromJsValue, attributes(qjs))]
pub fn derive_from_js_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    derive::derive(&mut input, true, false)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn host_call(attrs: TokenStream, input: TokenStream) -> TokenStream {
    host_fn::patch(
        syn::parse_macro_input!(attrs),
        syn::parse_macro_input!(input),
    )
    .into()
}

#[proc_macro_attribute]
pub fn qjsbind(attrs: TokenStream, input: TokenStream) -> TokenStream {
    qjsbind::patch(
        syn::parse_macro_input!(attrs),
        syn::parse_macro_input!(input),
    )
    .into()
}

fn find_crate_name(origin: &str) -> syn::Result<syn::Ident> {
    use proc_macro2::Span;
    use proc_macro_crate::{crate_name, FoundCrate};
    let name = match crate_name(origin) {
        Ok(FoundCrate::Itself) => syn::Ident::new("crate", Span::call_site()),
        Ok(FoundCrate::Name(alias)) => syn::Ident::new(&alias, Span::call_site()),
        Err(e) => {
            return Err(syn::Error::new(Span::call_site(), &e));
        }
    };
    Ok(name)
}
