use core::{ops::Deref, ptr};

use alloc::vec::Vec;
use anyhow::bail;

use crate::{self as js, c, error::expect_js_value, FromJsValue, GcMark, Result, ToJsValue, Value};

/// A wrapper of JS Uint8Array. When passing a string from JS to Rust, using this type
/// is more efficient than `Vec<u8>` because it avoids extra memory allocation and copy.
#[derive(Clone)]
pub struct JsArrayBuffer {
    value: Value,
    ptr: *const u8,
    len: usize,
}

impl core::fmt::Debug for JsArrayBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("JsArrayBuffer")
            .field("len", &self.len)
            .finish()
    }
}

impl JsArrayBuffer {
    pub fn new(ctx: &js::Context, len: usize) -> Result<Self> {
        let value = Value::new_moved(ctx, unsafe {
            c::JS_NewArrayBufferCopy(ctx.as_ptr(), ptr::null(), len as _)
        });
        Self::from_value(ctx, value)
    }

    pub fn from_value(ctx: &js::Context, value: Value) -> Result<Self> {
        unsafe {
            if value.is_exception() {
                bail!(
                    "failed to create ArrayBuffer: {:?}",
                    ctx.get_exception_error()
                );
            }
            let mut len = 0;
            let ptr = c::JS_GetArrayBuffer(ctx.as_ptr(), &mut len, *value.raw_value());
            if ptr.is_null() {
                bail!("failed to get ArrayBuffer ptr");
            }
            Ok(JsArrayBuffer {
                value,
                ptr: ptr as _,
                len,
            })
        }
    }
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn resized(&self, new_size: usize) -> Result<Self> {
        let new_buffer = JsArrayBuffer::new(self.value.context()?, new_size)?;
        let cp_size = core::cmp::min(self.len, new_size);
        new_buffer.fill_with_bytes(&self.as_bytes()[..cp_size]);
        Ok(new_buffer)
    }

    pub fn transfer(&self, new_len: usize) -> Result<Self> {
        if new_len > u32::MAX as usize {
            bail!("transfer new_len is too large");
        }
        let ctx = self.value.context()?;
        let new_len = new_len.to_js_value(&ctx)?;
        let new_value = self.value.call_method("transfer", &[new_len])?;
        Self::from_value(ctx, new_value)
    }
}

impl FromJsValue for JsArrayBuffer {
    fn from_js_value(value: Value) -> Result<Self> {
        if !value.is_array_buffer() {
            return Err(expect_js_value(&value, "ArrayBuffer"));
        }
        let ctx = value.context()?;
        let mut len = 0;
        let ptr = unsafe { c::JS_GetArrayBuffer(ctx.as_ptr(), &mut len, *value.raw_value()) };
        if ptr.is_null() {
            return Err(expect_js_value(&value, "Uint8Array"));
        }
        Ok(JsArrayBuffer {
            value,
            ptr: ptr as _,
            len,
        })
    }
}

impl ToJsValue for JsArrayBuffer {
    fn to_js_value(&self, _ctx: &js::Context) -> Result<Value> {
        Ok(self.value.clone())
    }
}

impl Deref for JsArrayBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl GcMark for JsArrayBuffer {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        self.value.gc_mark(rt, mark_fn);
    }
}
