use crate::{c, traits::ToNonNull, utils::js_throw_type_error, FromJsValue, ToJsValue, Value};
use core::ptr::NonNull;

pub trait HostFunction {
    fn call(
        &self,
        ctx: NonNull<c::JSContext>,
        this_val: c::JSValueConst,
        args: &[c::JSValue],
    ) -> c::JSValue;
}

pub struct Function<Ctx, Args, Ret, F> {
    f: F,
    _phantom: core::marker::PhantomData<(Ctx, Args, Ret)>,
}

pub fn call_host_function<Ctx, Args, Ret, F>(
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
    Function::new(func).call(
        ctx.to_non_null()
            .expect("calling host function with null context"),
        this_val,
        args,
    )
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

fn convert_result<V, E>(ctx: NonNull<c::JSContext>, recult: Result<V, E>) -> c::JSValue
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
            HF: Fn(Srv, Value, $($arg,)*) -> Ret,
            Srv: TryFrom<NonNull<c::JSContext>>,
            Srv::Error: core::fmt::Debug,
            $($arg: FromJsValue,)*
            Ret: ToJsValue,
        {
            fn call(&self, ctx: NonNull<c::JSContext>, this_val: c::JSValueConst, args: &[c::JSValue]) -> c::JSValue {
                #[allow(non_snake_case)]
                Function::new(|srv: Srv, this_value, $($arg: $arg,)*| -> Result<Ret, ()> {
                    Ok((self.f)(srv, this_value, $($arg,)*))
                }).call(ctx, this_val, args)
            }
        }
        impl<HF, Srv, $($arg,)* Ret, Err> HostFunction for Function<Srv, ($($arg,)*), Result<Ret, Err>, HF>
        where
            HF: Fn(Srv, Value, $($arg,)*) -> Result<Ret, Err>,
            Srv: TryFrom<NonNull<c::JSContext>>,
            Srv::Error: core::fmt::Debug,
            Err: core::fmt::Debug,
            $($arg: FromJsValue,)*
            Ret: ToJsValue,
        {
            fn call(&self, ctx: NonNull<c::JSContext>, this_val: c::JSValueConst, args: &[c::JSValue]) -> c::JSValue {
                let this_value = Value::new_cloned(ctx, this_val);
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

pub extern "C" fn host_fn_stub<Src, F>(
    ctx: *mut c::JSContext,
    this_val: c::JSValueConst,
    argc: core::ffi::c_int,
    argv: *const c::JSValueConst,
) -> c::JSValue
where
    Src: TryFrom<NonNull<c::JSContext>>,
    Src::Error: core::fmt::Debug,
    F: HostFunction + Default,
{
    let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
    let func = F::default();
    func.call(
        ctx.to_non_null().expect("host call with null context"),
        this_val,
        args,
    )
}