use core::ptr::NonNull;

use crate::{c, Error, FromJsValue, Result, ToJsValue, Value};

/// A wrapper of JS Uint8Array. When passing a string from JS to Rust, using this type
/// is more efficient than `Vec<u8>` because it avoids extra memory allocation and copy.
pub struct JsUint8Array {
    value: Value,
    ptr: *const u8,
    len: usize,
}

impl JsUint8Array {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl FromJsValue for JsUint8Array {
    fn from_js_value(value: Value) -> Result<Self> {
        if !value.is_uint8_array() {
            return Err(Error::Expect("Uint8Array"));
        }
        let mut len = 0;
        let ptr = unsafe { c::JS_Uint8ArrayGetBuffer(*value.raw_value(), &mut len) };
        if ptr.is_null() {
            return Err(Error::Expect("Uint8Array"));
        }
        Ok(JsUint8Array {
            value,
            ptr: ptr as _,
            len,
        })
    }
}

impl ToJsValue for JsUint8Array {
    fn to_js_value(&self, _ctx: NonNull<c::JSContext>) -> Result<Value> {
        Ok(self.value.clone())
    }
}
