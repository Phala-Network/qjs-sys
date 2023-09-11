use crate::{self as js, c, FromJsValue, ToJsValue, Value};

pub trait HostFunction {
    fn call(&self, ctx: &js::Context, this_val: c::JSValueConst, args: &[c::JSValue])
        -> c::JSValue;
}

pub struct Function<Ctx, Args, Ret, F> {
    f: F,
    _phantom: core::marker::PhantomData<(Ctx, Args, Ret)>,
}

/// Call a host function from JavaScript.
///
/// # Safety
/// `ctx` must be a valid pointer to a `JSContext`.
/// `argv` must be a valid pointer to an array of `argc` `JSValue`s.
pub unsafe fn call_host_function<Ctx, Args, Ret, F>(
    func: F,
    ctx: *mut c::JSContext,
    this_val: c::JSValueConst,
    argc: core::ffi::c_int,
    argv: *const c::JSValue,
) -> c::JSValue
where
    Function<Ctx, Args, Ret, F>: HostFunction,
{
    let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
    let ctx = js::Context::clone_from_ptr(ctx).expect("calling host function with null context");
    Function::new(func).call(&ctx, this_val, args)
}

impl<Ctx, Args, Ret, F> Function<Ctx, Args, Ret, F> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<Ctx, Args, Ret, F: Default> Default for Function<Ctx, Args, Ret, F> {
    fn default() -> Self {
        Self {
            f: Default::default(),
            _phantom: core::marker::PhantomData,
        }
    }
}

fn convert_result<V, E>(ctx: &js::Context, recult: Result<V, E>) -> c::JSValue
where
    V: ToJsValue,
    E: core::fmt::Debug,
{
    match recult {
        Ok(v) => match v.to_js_value(ctx) {
            Ok(v) => v.leak(),
            Err(err) => {
                let msg = format!(
                    "failed to convert {} to JsValue: {err:?}",
                    tynm::type_name::<V>()
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
        impl<HF, Srv, $($arg,)* Ret> HostFunction for Function<Srv, ($($arg,)*), Ret, HF>
        where
            HF: Fn(Srv, Value, $($arg,)*) -> Ret,
            Srv: TryFrom<js::Context>,
            Srv::Error: core::fmt::Debug,
            $($arg: FromJsValue,)*
            Ret: ToJsValue,
        {
            fn call(&self, ctx: &js::Context, this_val: c::JSValueConst, args: &[c::JSValue]) -> c::JSValue {
                #[allow(non_snake_case)]
                Function::new(|srv: Srv, this_value, $($arg: $arg,)*| -> Result<Ret, ()> {
                    Ok((self.f)(srv, this_value, $($arg,)*))
                }).call(ctx, this_val, args)
            }
        }
        impl<HF, Srv, $($arg,)* Ret, Err> HostFunction for Function<Srv, ($($arg,)*), Result<Ret, Err>, HF>
        where
            HF: Fn(Srv, Value, $($arg,)*) -> Result<Ret, Err>,
            Srv: TryFrom<js::Context>,
            Srv::Error: core::fmt::Debug,
            Err: core::fmt::Debug,
            $($arg: FromJsValue,)*
            Ret: ToJsValue,
        {
            fn call(&self, ctx: &js::Context, this_val: c::JSValueConst, args: &[c::JSValue]) -> c::JSValue {
                let this_value = Value::new_cloned(ctx, this_val);
                let srv = match Srv::try_from(ctx.clone()) {
                    Ok(ctx) => ctx,
                    Err(e) => {
                        let msg = format!(
                            "failed to convert JsContext to {}: {:?}",
                            tynm::type_name::<Srv>(),
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
                            let msg = format!("failed to convert JsValue to {}: {err:?}", tynm::type_name::<$arg>());
                            ctx.throw_type_err(&msg);
                            return c::JS_EXCEPTION;
                        }
                    };
                )*
                let ret = (self.f)(srv, this_value, $($arg,)*);
                convert_result(ctx, ret)
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
