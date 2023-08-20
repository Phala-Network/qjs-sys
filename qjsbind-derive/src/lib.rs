use proc_macro::TokenStream;
use syn::parse_macro_input;

mod attrs;
mod bound;
mod derive;

#[proc_macro_derive(ToJsValue, attributes(jsbind))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    derive::derive(&mut input, false)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(FromJsValue, attributes(jsbind))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    derive::derive(&mut input, true)
        .unwrap_or_else(|err| err.to_compile_error())
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