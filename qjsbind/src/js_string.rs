use alloc::string::{String, ToString};
use core::{ffi::CStr, fmt::Debug};

use crate::{self as js, c, Error, FromJsValue, Result, ToJsValue, Value};

/// A wrapper of JS string. When passing a string from JS to Rust, using this type
/// is more efficient than `String` because it avoids extra memory allocation and copy.
pub struct JsString {
    value: Value,
    ptr: *const u8,
    len: usize,
}

impl Debug for JsString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("JsString")
            .field("", &self.as_str())
            .finish()
    }
}

impl JsString {
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.ptr, self.len);
            core::str::from_utf8_unchecked(slice)
        }
    }
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr as _) }
    }
}

impl ToString for JsString {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Drop for JsString {
    fn drop(&mut self) {
        let ctx = self
            .value
            .context()
            .expect("BUG: context is null for a JsString");
        unsafe {
            c::JS_FreeCString(ctx.as_ptr(), self.ptr as _);
        }
    }
}

impl FromJsValue for JsString {
    fn from_js_value(value: Value) -> Result<Self> {
        let ctx = value.context().or(Err(Error::Expect("Context")))?;
        if !value.is_string() {
            return Err(Error::Expect("string"));
        }
        let mut len = 0;
        let ptr = unsafe { c::JS_ToCStringLen(ctx.as_ptr(), &mut len, *value.raw_value()) };
        if ptr.is_null() {
            return Err(Error::Expect("string"));
        }
        Ok(JsString {
            value,
            ptr: ptr as _,
            len,
        })
    }
}

impl ToJsValue for JsString {
    fn to_js_value(&self, _ctx: &js::Context) -> Result<Value> {
        Ok(self.value.clone())
    }
}
