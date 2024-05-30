use crate::{self as js, c, FromJsValue, IntoJsValue, Value};

pub trait HostFunction {
    fn call(&self, ctx: &js::Context, this_val: c::JSValueConst, args: &[c::JSValue])
        -> c::JSValue;
}

pub struct Function<Ctx, This, Args, Ret, F> {
    name: &'static str,
    f: F,
    _phantom: core::marker::PhantomData<(Ctx, This, Args, Ret)>,
}

/// Call a host function from JavaScript.
///
/// # Safety
/// `ctx` must be a valid pointer to a `JSContext`.
/// `argv` must be a valid pointer to an array of `argc` `JSValue`s.
pub unsafe fn call_host_function<Ctx, This, Args, Ret, F>(
    name: &'static str,
    func: F,
    ctx: *mut c::JSContext,
    this_val: c::JSValueConst,
    argc: core::ffi::c_int,
    argv: *const c::JSValue,
) -> c::JSValue
where
    Function<Ctx, This, Args, Ret, F>: HostFunction,
{
    let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
    let ctx = js::Context::clone_from_ptr(ctx).expect("calling host function with null context");
    Function::new(name, func).call(&ctx, this_val, args)
}

impl<Ctx, This, Args, Ret, F> Function<Ctx, This, Args, Ret, F> {
    pub fn new(name: &'static str, f: F) -> Self {
        Self {
            name,
            f,
            _phantom: core::marker::PhantomData,
        }
    }
}

fn convert_result<V, E>(fname: &str, ctx: &js::Context, recult: Result<V, E>) -> c::JSValue
where
    V: IntoJsValue,
    E: core::fmt::Debug,
{
    match recult {
        Ok(v) => match v.into_js_value(ctx) {
            Ok(v) => v.leak(),
            Err(err) => {
                let msg = format!(
                    "[{fname}] failed to convert {} to JsValue: {err:?}",
                    crate::type_name::<V>()
                );
                ctx.throw_str(&msg);
                c::JS_EXCEPTION
            }
        },
        Err(err) => {
            ctx.throw_dbg(err);
            c::JS_EXCEPTION
        }
    }
}

macro_rules! impl_host_fn {
    (($($arg:ident),*)) => {
        impl<HF, This, Srv, $($arg,)* Ret> HostFunction for Function<Srv, This, ($($arg,)*), Ret, HF>
        where
            HF: Fn(Srv, This, $($arg,)*) -> Ret,
            Srv: TryFrom<js::Context>,
            Srv::Error: core::fmt::Debug,
            This: FromJsValue,
            $($arg: FromJsValue,)*
            Ret: IntoJsValue,
        {
            fn call(&self, ctx: &js::Context, this_val: c::JSValueConst, args: &[c::JSValue]) -> c::JSValue {
                #[allow(non_snake_case)]
                Function::new(self.name, |srv: Srv, this_value, $($arg: $arg,)*| -> Result<Ret, js::Error> {
                    Ok((self.f)(srv, FromJsValue::from_js_value(this_value)?, $($arg,)*))
                }).call(ctx, this_val, args)
            }
        }
        impl<HF, This, Srv, $($arg,)* Ret, Err> HostFunction for Function<Srv, This, ($($arg,)*), Result<Ret, Err>, HF>
        where
            HF: Fn(Srv, This, $($arg,)*) -> Result<Ret, Err>,
            Srv: TryFrom<js::Context>,
            Srv::Error: core::fmt::Debug,
            Err: core::fmt::Debug,
            This: FromJsValue,
            $($arg: FromJsValue,)*
            Ret: IntoJsValue,
        {
            fn call(&self, ctx: &js::Context, this_val: c::JSValueConst, args: &[c::JSValue]) -> c::JSValue {
                let this_value = match FromJsValue::from_js_value(Value::new_cloned(ctx, this_val)) {
                    Ok(this_value) => this_value,
                    Err(err) => {
                        let msg = format!(
                            "[{}] failed to convert JsValue to {}: {err:?}",
                            self.name,
                            crate::type_name::<This>(),
                        );
                        ctx.throw_type_err(&msg);
                        return c::JS_EXCEPTION;
                    }
                };

                let srv = match Srv::try_from(ctx.clone()) {
                    Ok(ctx) => ctx,
                    Err(e) => {
                        let msg = format!(
                            "[{}] failed to convert JsContext to {}: {:?}",
                            self.name,
                            crate::type_name::<Srv>(),
                            e
                        );
                        ctx.throw_type_err(&msg);
                        return c::JS_EXCEPTION;
                    }
                };
                #[allow(unused_variables)]
                #[allow(unused_mut)]
                let mut args = args.iter();
                $(
                    let value = Value::new_cloned(ctx, *args.next().unwrap_or(&c::JS_UNDEFINED));
                    #[allow(non_snake_case)]
                    let $arg = match $arg::from_js_value(value) {
                        Ok(arg) => arg,
                        Err(err) => {
                            let msg = format!(
                                "[{}] failed to convert JsValue to {}: {err:?}",
                                self.name,
                                crate::type_name::<$arg>()
                            );
                            ctx.throw_type_err(&msg);
                            return c::JS_EXCEPTION;
                        }
                    };
                )*
                let ret = (self.f)(srv, this_value, $($arg,)*);
                convert_result(self.name, ctx, ret)
            }
        }
    };
}

impl_host_fn!(());
impl_host_fn!((A));
impl_host_fn!((A, B));
impl_host_fn!((A, B, C));
impl_host_fn!((A, B, C, D));
impl_host_fn!((A, B, C, D, E));
impl_host_fn!((A, B, C, D, E, F));
impl_host_fn!((A, B, C, D, E, F, G));
impl_host_fn!((A, B, C, D, E, F, G, H));
