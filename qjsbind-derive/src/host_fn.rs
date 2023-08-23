use proc_macro2::TokenStream;

pub(crate) fn patch(input: TokenStream) -> TokenStream {
    match patch_or_err(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn patch_or_err(input: TokenStream) -> syn::Result<TokenStream> {
    let the_fn: syn::ItemFn = syn::parse2(input.clone())?;
    let fn_ident = &the_fn.sig.ident;
    let crate_qjsbind = crate::find_crate_name("qjsbind")?;
    Ok(syn::parse_quote! {
        pub extern "C" fn #fn_ident(
            ctx: *mut #crate_qjsbind::c::JSContext,
            this_val: #crate_qjsbind::c::JSValueConst,
            argc: core::ffi::c_int,
            argv: *mut #crate_qjsbind::c::JSValue,
        ) -> #crate_qjsbind::c::JSValue
        {
            #input
            #crate_qjsbind::call_host_function(#fn_ident, ctx, this_val, argc, argv)
        }
    })
}
