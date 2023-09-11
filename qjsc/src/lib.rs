use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::spanned::Spanned;

#[proc_macro]
pub fn compiled(input: TokenStream) -> TokenStream {
    match compile_js(syn::parse_macro_input!(input)) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn compile_js(js: syn::LitStr) -> syn::Result<TokenStream2> {
    let js = js.value();
    match qjsbind::compile(&js, "<eval>") {
        Ok(bytecode) => {
            let lit_bytes = syn::LitByteStr::new(&bytecode, js.span());
            Ok(quote::quote! {
                #lit_bytes
            })
        }
        Err(err) => {
            let msg = format!("{}: {}", err, js);
            Err(syn::Error::new(js.span(), msg))
        }
    }
}
