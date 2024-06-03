use js::AnyError;

use crate::{self as js, c, ToJsValue, Value};

mod private {
    pub trait Sealed {}
}

pub trait HostCallOutput: private::Sealed {
    fn into_js_value(self, ctx: &js::Context) -> js::Result<Value>;
}

impl<T> private::Sealed for T where T: ToJsValue {}
impl<T> HostCallOutput for T
where
    T: ToJsValue,
{
    fn into_js_value(self, ctx: &js::Context) -> js::Result<Value> {
        self.to_js_value(ctx)
    }
}

impl<T, E> private::Sealed for Result<T, E>
where
    T: HostCallOutput,
    E: AnyError,
{
}
impl<T, E> HostCallOutput for Result<T, E>
where
    T: HostCallOutput,
    E: AnyError,
{
    fn into_js_value(self, ctx: &js::Context) -> js::Result<Value> {
        self.map_err(js::Error::msg)?.into_js_value(ctx)
    }
}

pub fn convert_host_call_result(
    _fname: &str,
    ctx: &js::Context,
    result: impl HostCallOutput,
) -> c::JSValue {
    match result.into_js_value(ctx) {
        Ok(v) => v.leak(),
        Err(err) => {
            ctx.throw_dbg(&err);
            c::JS_EXCEPTION
        }
    }
}
