use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;

use crate::c;

pub fn js_val_into_u128(ctx: *mut c::JSContext, v: c::JSValue) -> Result<u128, &'static str> {
    scopeguard::defer! {
        unsafe { c::JS_FreeValue(ctx, v) };
    }
    js_val_to_u128(ctx, v)
}

pub fn js_val_to_u128(ctx: *mut c::JSContext, v: c::JSValue) -> Result<u128, &'static str> {
    unsafe {
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
    scopeguard::defer! {
        unsafe { c::JS_FreeValue(ctx, v) };
    }
    js_val_to_bytes(ctx, v)
}

pub fn js_val_to_bytes(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Vec<u8>, &'static str> {
    unsafe {
        if c::JS_IsString(v) != 0 {
            let cstr = c::JS_ToCString(ctx, v);
            if cstr.is_null() {
                return Err("invalid string");
            }
            let cstr = CStr::from_ptr(cstr);
            let str = cstr.to_string_lossy();
            match hex::decode(str.strip_prefix("0x").unwrap_or(&str)) {
                Ok(bytes) => Ok(bytes),
                Err(_) => Err("failed to decode bytes in hex string"),
            }
        } else {
            let mut size = 0;
            let ptr = c::JS_Uint8ArrayGetBuffer(v, &mut size);
            if ptr.is_null() {
                return Err("not a Uint8Array");
            }
            let buf = core::slice::from_raw_parts(ptr, size as _);
            Ok(buf.to_vec())
        }
    }
}

pub fn js_val_into_string(ctx: *mut c::JSContext, v: c::JSValue) -> Result<String, &'static str> {
    scopeguard::defer! {
        unsafe { c::JS_FreeValue(ctx, v) };
    }
    js_val_to_string(ctx, v)
}

pub fn js_val_to_string(ctx: *mut c::JSContext, v: c::JSValue) -> Result<String, &'static str> {
    unsafe {
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

pub fn js_val_from_bytes(ctx: *mut c::JSContext, bytes: &[u8]) -> c::JSValue {
    return unsafe { c::JS_Uint8ArrayCreate(ctx, bytes.as_ptr() as _, bytes.len() as _) };
}
