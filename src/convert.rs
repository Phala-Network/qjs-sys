use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;

use crate::c;

pub fn js_val_into_u128(ctx: *mut c::JSContext, v: c::JSValue) -> Result<u128, &'static str> {
    unsafe {
        scopeguard::defer! {
            c::JS_FreeValue(ctx, v);
        }

        if c::JS_IsNumber(v) != 0 {
            let mut val = 0;
            c::JS_ToUint32(ctx, &mut val, v);
            return Ok(val as u128);
        }

        let cstr = c::JS_ToCString(ctx, v);
        if cstr.is_null() {
            return Err("failed to convert to string");
        }

        scopeguard::defer! {
            c::JS_FreeCString(ctx, cstr);
        }

        let cstr = CStr::from_ptr(cstr);
        let str = cstr.to_string_lossy();
        let val = str.parse::<u128>().or(Err("not a number"))?;

        Ok(val)
    }
}

pub fn js_val_into_bytes(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Vec<u8>, &'static str> {
    unsafe {
        scopeguard::defer! {
            c::JS_FreeValue(ctx, v);
        }

        let mut size = 0;
        let ptr = c::JS_Uint8ArrayGetBuffer(v, &mut size);
        if ptr.is_null() {
            return Err("not a Uint8Array");
        }
        let buf = core::slice::from_raw_parts(ptr, size as _);
        Ok(buf.to_vec())
    }
}

pub fn js_val_into_string(ctx: *mut c::JSContext, v: c::JSValue) -> Result<String, &'static str> {
    unsafe {
        scopeguard::defer! {
            c::JS_FreeValue(ctx, v);
        }

        let cstr = c::JS_ToCString(ctx, v);
        if cstr.is_null() {
            return Err("not a string");
        }
        scopeguard::defer! {
            c::JS_FreeCString(ctx, cstr);
        }

        let cstr = CStr::from_ptr(cstr);
        let str = cstr.to_string_lossy();

        Ok(str.into_owned())
    }
}
