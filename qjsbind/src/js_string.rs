use core::{
    ffi::CStr,
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::{self as js, c, error::expect_js_value, FromJsValue, GcMark, Result, ToJsValue, Value};

/// A wrapper of JS string. When passing a string from JS to Rust, using this type
/// is more efficient than `String` because it avoids extra memory allocation and copy.
#[derive(Clone)]
pub struct JsString {
    value: Value,
    ptr: *const u8,
    len: usize,
}

impl GcMark for JsString {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        self.value.gc_mark(rt, mark_fn);
    }
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
        let ctx = value.context()?;
        if !value.is_string() {
            return Err(expect_js_value(&value, "string"));
        }
        let mut len = 0;
        let ptr = unsafe { c::JS_ToCStringLen(ctx.as_ptr(), &mut len, *value.raw_value()) };
        if ptr.is_null() {
            return Err(expect_js_value(&value, "string"));
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

#[derive(Debug)]
pub enum String {
    Native(alloc::string::String),
    JsString(JsString),
}

impl GcMark for String {
    fn gc_mark(&self, rt: *mut js::c::JSRuntime, mark_fn: js::c::JS_MarkFunc) {
        match self {
            Self::Native(_) => {}
            Self::JsString(s) => s.gc_mark(rt, mark_fn),
        }
    }
}

impl From<alloc::string::String> for String {
    fn from(s: alloc::string::String) -> Self {
        Self::Native(s)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        Self::Native(s.into())
    }
}

impl From<JsString> for String {
    fn from(s: JsString) -> Self {
        Self::JsString(s)
    }
}

impl From<String> for alloc::string::String {
    fn from(s: String) -> Self {
        match s {
            String::Native(s) => s,
            String::JsString(s) => s.as_str().into(),
        }
    }
}

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Native(s) => s.as_str(),
            Self::JsString(s) => s.as_str(),
        }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromJsValue for String {
    fn from_js_value(value: Value) -> Result<Self> {
        Ok(JsString::from_js_value(value)?.into())
    }
}

impl ToJsValue for String {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        match self {
            Self::Native(s) => s.to_js_value(ctx),
            Self::JsString(s) => s.to_js_value(ctx),
        }
    }
}

impl String {
    pub fn as_str(&self) -> &str {
        self.deref()
    }
}
