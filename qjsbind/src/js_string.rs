use core::{
    ffi::CStr,
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::{self as js, c, Error, FromJsValue, Result, ToJsValue, Value};

/// A wrapper of JS string. When passing a string from JS to Rust, using this type
/// is more efficient than `String` because it avoids extra memory allocation and copy.
#[derive(Clone)]
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

impl Display for JsString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
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
        let js_value = unsafe { c::JS_CStringOuterValue(ctx.as_ptr(), ptr) };
        let value = Value::new_moved(ctx, js_value);

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

impl Deref for JsString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
