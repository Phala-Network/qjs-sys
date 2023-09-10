use proc_macro2::TokenStream;
use template_quote::quote;

pub(crate) fn patch(input: TokenStream, with_context: bool) -> TokenStream {
    match patch_or_err(input, with_context) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn patch_or_err(input: TokenStream, with_context: bool) -> syn::Result<TokenStream> {
    let the_fn: syn::ItemFn = syn::parse2(input.clone())?;
    let fn_ident = &the_fn.sig.ident;
    let crate_qjsbind = crate::find_crate_name("qjsbind")?;
    let args = the_fn.sig.inputs;
    let arg_names = args
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat) => match &*pat.pat {
                syn::Pat::Ident(ref ident) => Some(ident.ident.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();
    let output = match the_fn.sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };
    Ok(quote! {
        pub unsafe extern "C" fn #fn_ident(
            ctx: *mut #crate_qjsbind::c::JSContext,
            this_val: #crate_qjsbind::c::JSValueConst,
            argc: core::ffi::c_int,
            argv: *mut #crate_qjsbind::c::JSValue,
        ) -> #crate_qjsbind::c::JSValue
        {
            #input
            #(if with_context) {
                #crate_qjsbind::call_host_function(#fn_ident, ctx, this_val, argc, argv)
            }
            #(else) {
                fn wrapper(_ctx: #crate_qjsbind::Context, _this: #crate_qjsbind::Value, #args) -> #output {
                    #fn_ident(#(#arg_names),*)
                }
                #crate_qjsbind::call_host_function(wrapper, ctx, this_val, argc, argv)
            }
        }
    })
}
