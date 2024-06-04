use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{parse::Parser, parse_quote, spanned::Spanned, Ident};
use template_quote::quote;

use crate::attrs::respan;

pub(crate) fn patch(attrs: TokenStream, input: TokenStream) -> TokenStream {
    match patch_or_err(attrs, input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn patch_or_err(attrs: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let mut with_context = false;
    syn::meta::parser(|meta| {
        if meta.path.is_ident("with_context") {
            with_context = true;
        }
        Ok(())
    })
    .parse2(attrs)?;

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
    let ctx_var;
    let this_var;
    let mut arg_exprs = Vec::new();
    let mut args_iter = arg_names.into_iter();
    if with_context {
        let Some(ctx) = args_iter.next() else {
            syn_bail!(args, "missing context argument");
        };
        ctx_var = quote_spanned! { ctx.span() => ctx };
        arg_exprs.push(quote_spanned! { ctx.span() =>
             #crate_qjsbind::ErrorContext::context(#ctx_var.try_into().ok(), "failed to convert context")?
        });
        let Some(this) = args_iter.next() else {
            syn_bail!(args, "missing this argument");
        };
        this_var = quote_spanned! {this.span() => this_value };
        arg_exprs.push(respan(
            this.span(),
            quote! { #crate_qjsbind::FromJsValue::from_js_value(#this_var)? },
        ));
    } else {
        ctx_var = parse_quote!(ctx);
        this_var = parse_quote!(this_value);
    }
    for arg in args_iter {
        arg_exprs.push(respan(arg.span(), quote! {
            #crate_qjsbind::FromJsValue::from_js_value(args.next().unwrap_or(#crate_qjsbind::Value::undefined()))?
        }));
    }
    let fn_name = fn_ident.to_string();
    let rv = Ident::new("rv", the_fn.sig.output.span());
    Ok(quote! {
        pub unsafe extern "C" fn #fn_ident(
            c_ctx: *mut #crate_qjsbind::c::JSContext,
            c_this: #crate_qjsbind::c::JSValueConst,
            argc: core::ffi::c_int,
            argv: *mut #crate_qjsbind::c::JSValue,
        ) -> #crate_qjsbind::c::JSValue
        {
            #input
            #crate_qjsbind::log::trace!(target: "js::ocall", "host function {} called, argc={argc}", #fn_name);
            #[allow(unused_variables)]
            let #ctx_var = #crate_qjsbind::Context::clone_from_ptr(c_ctx).expect("calling host function with null context");
            let _pause_gc = #ctx_var.pause_gc();
            let args = if argc > 0 {
                unsafe { core::slice::from_raw_parts(argv, argc as usize) }
            } else {
                &[]
            };
            let mut args = args.into_iter().map(|v| #crate_qjsbind::Value::new_cloned(&ctx, *v));
            #(if with_context) {
                let #this_var = #crate_qjsbind::Value::new_cloned(&ctx, c_this);
            }
            #(else) {
                let _ = c_this;
            }
            let #rv: #crate_qjsbind::Result<_> = {
                #(if with_context) {

                let ctx = ctx.clone();

                }
                (move|| { Ok(#fn_ident(#(#arg_exprs),*)) })()
            };
            #crate_qjsbind::convert_host_call_result(#fn_name, &#ctx_var, #rv)
        }
    })
}

#[test]
fn show_tokens() {
    let tokens = quote! {
    fn codec(
        ctx: js::Context,
        _this: js::Value,
        tid: js::Value,
        registry: js::Value,
    ) -> js::Result<js::Value> {
        let obj = ctx.new_object("ScaleCodec");
        let proto = ctx.get_global_object().get_property("ScaleCodec")?;
        obj.set_prototype(&proto)?;
        obj.set_property("ty", &tid)?;
        obj.set_property("registry", &registry)?;
        obj.set_property("isArray", &js::Value::from_bool(&ctx, tid.is_array()))?;
        Ok(obj)
    }
        };
    let patched = patch(quote!(with_context), tokens);
    insta::assert_snapshot!(rustfmt_snippet::rustfmt(&patched.to_string()).unwrap());
}
