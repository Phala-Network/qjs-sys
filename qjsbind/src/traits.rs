use crate as js;

use super::{c, Result, Value};
use crate::value::RawValue;
use tinyvec::TinyVec;

pub struct OwnedRawArgs {
    _args: TinyVec<[Value; 8]>,
    raw_args: TinyVec<[RawValue; 8]>,
}

impl OwnedRawArgs {
    pub fn as_mut_ptr(&mut self) -> *mut c::JSValue {
        self.raw_args.as_mut_ptr() as *mut c::JSValue
    }
    pub fn as_ptr(&self) -> *const c::JSValue {
        self.raw_args.as_ptr() as *const c::JSValue
    }
    pub fn len(&self) -> usize {
        self.raw_args.len()
    }
    pub fn is_empty(&self) -> bool {
        self.raw_args.is_empty()
    }
}

pub trait FromJsContext {
    fn from_js_context(ctx: &js::Context) -> Result<Self>
    where
        Self: Sized;
}

pub trait FromJsValue {
    fn from_js_value(js_value: Value) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToJsValue {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value>;
}

impl ToJsValue for &dyn ToJsValue {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        (*self).to_js_value(ctx)
    }
}

pub trait FromArgs {
    fn from_args(argv: &[Value]) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToArgs {
    fn to_args(&self, ctx: &js::Context) -> Result<TinyVec<[Value; 8]>>;

    fn to_raw_args(&self, ctx: &js::Context) -> Result<OwnedRawArgs> {
        let args = self.to_args(ctx)?;
        let raw_args = args.iter().map(|v| RawValue(*v.raw_value())).collect();
        Ok(OwnedRawArgs {
            _args: args,
            raw_args,
        })
    }
}
