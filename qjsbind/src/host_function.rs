use crate::{c, utils::js_throw_type_error, FromJsValue, ToJsValue, Value};

pub trait HostFunction {
    fn call(&self, ctx: *mut c::JSContext, args: &[c::JSValue]) -> c::JSValue;
}

pub struct Function<Ctx, Args, Ret, F> {
    f: F,
    _phantom: core::marker::PhantomData<(Ctx, Args, Ret)>,
}

pub fn call_host_function<Ctx, Args, Ret, F>(
    func: F,
    ctx: *mut c::JSContext,
    args: &[c::JSValue],
) -> c::JSValue
where
    Function<Ctx, Args, Ret, F>: HostFunction,
{
    Function::new(func).call(ctx, args)
}

impl<Ctx, Args, Ret, F> Function<Ctx, Args, Ret, F> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: core::marker::PhantomData,
        }
    }
}

fn convert_result<V, E>(ctx: *mut c::JSContext, recult: Result<V, E>) -> c::JSValue
where
    V: ToJsValue,
    E: core::fmt::Debug,
{
    match recult {
        Ok(v) => match v.to_js_value(ctx) {
            Ok(v) => v.into_raw(),
            Err(err) => {
                let msg = format!("failed to convert to JSValue: {err:?}");
                js_throw_type_error(ctx, &msg);
                c::JS_EXCEPTION
            }
        },
        Err(err) => {
            let msg = format!("{err:?}");
            js_throw_type_error(ctx, &msg);
            c::JS_EXCEPTION
        }
    }
}

macro_rules! impl_host_fn {
    (($($arg:ident),*)) => {
        impl<HF, Srv, $($arg,)* Ret> HostFunction for Function<Srv, ($($arg,)*), Ret, HF>
        where
            HF: Fn(Srv, $($arg,)*) -> Ret,
            Srv: TryFrom<*mut c::JSContext>,
            Srv::Error: core::fmt::Debug,
            $($arg: FromJsValue,)*
            Ret: ToJsValue,
        {
            fn call(&self, ctx: *mut c::JSContext, args: &[c::JSValue]) -> c::JSValue {
                #[allow(non_snake_case)]
                Function::new(|srv: Srv, $($arg: $arg,)*| -> Result<Ret, ()> {
                    Ok((self.f)(srv, $($arg,)*))
                }).call(ctx, args)
            }
        }
        impl<HF, Srv, $($arg,)* Ret, Err> HostFunction for Function<Srv, ($($arg,)*), Result<Ret, Err>, HF>
        where
            HF: Fn(Srv, $($arg,)*) -> Result<Ret, Err>,
            Srv: TryFrom<*mut c::JSContext>,
            Srv::Error: core::fmt::Debug,
            Err: core::fmt::Debug,
            $($arg: FromJsValue,)*
            Ret: ToJsValue,
        {
            fn call(&self, ctx: *mut c::JSContext, args: &[c::JSValue]) -> c::JSValue {
                let srv = match Srv::try_from(ctx) {
                    Ok(ctx) => ctx,
                    Err(e) => {
                        let msg = format!(
                            "failed to convert JSContext to {}: {:?}",
                            core::any::type_name::<Srv>(),
                            e
                        );
                        js_throw_type_error(ctx, &msg);
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
                            let msg = format!("failed to convert JSValue to {}: {:?}", core::any::type_name::<$arg>(), err);
                            js_throw_type_error(ctx, &msg);
                            return c::JS_EXCEPTION;
                        }
                    };
                )*
                let ret = (self.f)(srv, $($arg,)*);
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
