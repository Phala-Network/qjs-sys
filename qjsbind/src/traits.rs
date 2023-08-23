use super::{c, Result, Value};

pub trait FromJsValue {
    fn from_js_value(js_value: Value) -> Result<Self>
    where
        Self: Sized;
}
pub trait ToJsValue {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value>;
}

pub trait FromArgs {
    fn from_args(argv: &[Value]) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToArgs {
    fn to_args(&self, ctx: *mut c::JSContext) -> Result<tinyvec::TinyVec<[Value; 8]>>;
}
