use core::ops::Deref;

use alloc::vec::Vec;

use crate::{self as js, c, Error, FromJsValue, GcMark, Result, ToJsValue, Value};

/// A wrapper of JS Uint8Array. When passing a string from JS to Rust, using this type
/// is more efficient than `Vec<u8>` because it avoids extra memory allocation and copy.
#[derive(Clone)]
pub struct JsUint8Array {
    value: Value,
    ptr: *const u8,
    len: usize,
}

impl core::fmt::Debug for JsUint8Array {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("JsUint8Array")
            .field("len", &self.len)
            .finish()
    }
}

impl JsUint8Array {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn fill_with_bytes(&self, bytes: &[u8]) -> bool {
        if bytes.len() > self.len {
            return false;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr as _, bytes.len());
        }
        true
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
    fn to_js_value(&self, _ctx: &js::Context) -> Result<Value> {
        Ok(self.value.clone())
    }
}

impl Deref for JsUint8Array {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl GcMark for JsUint8Array {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        self.value.gc_mark(rt, mark_fn);
    }
}
