use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use core::ffi::{c_char, CStr};

use crate::c;
use c::{
    JS_TAG_BIG_INT as TAG_BIG_INT, JS_TAG_BOOL as TAG_BOOL, JS_TAG_EXCEPTION as TAG_EXCEPTION,
    JS_TAG_FLOAT64 as TAG_FLOAT64, JS_TAG_INT as TAG_INT, JS_TAG_NULL as TAG_NULL,
    JS_TAG_OBJECT as TAG_OBJECT, JS_TAG_STRING as TAG_STRING, JS_TAG_UNDEFINED as TAG_UNDEFINED,
};

pub enum JsValue {
    Undefined,
    Null,
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
    Array(Vec<JsValue>),
    Bytes(Vec<u8>),
    Object(BTreeMap<String, JsValue>),
    BigInt(String),
    Raw(c::JSValue),
}

pub trait DecodeFromJSValue {
    fn decode(ctx: NonNull<c::JSContext>, v: c::JSValue) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn decode_into(ctx: NonNull<c::JSContext>, v: c::JSValue) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        scopeguard::defer! {
            unsafe { c::JS_FreeValue(ctx, v) };
        }
        Self::decode(ctx, v)
    }
}

pub fn js_object_get_field<T: DecodeFromJSValue>(
    ctx: NonNull<c::JSContext>,
    v: c::JSValue,
    field: &str,
) -> Result<T, String> {
    let value = js_object_get_option_field(ctx, v, field)?;
    value.ok_or_else(|| alloc::format!("missing field '{field}'"))
}

pub fn js_object_get_option_field<T: DecodeFromJSValue>(
    ctx: NonNull<c::JSContext>,
    v: c::JSValue,
    field: &str,
) -> Result<Option<T>, String> {
    let cfield =
        alloc::ffi::CString::new(field).or(Err(alloc::format!("invalid field name: '{field}'")))?;
    let value = unsafe { c::JS_GetPropertyStr(ctx, v, cfield.as_ptr()) };
    if unsafe { c::JS_IsUndefined(value) != 0 } {
        return Ok(None);
    }
    Ok(Some(DecodeFromJSValue::decode_into(ctx, value).or(Err(
        alloc::format!("invalid value for field '{field}'"),
    ))?))
}

pub fn js_array_for_each<F: FnMut(c::JSValue) -> Result<(), String>>(
    ctx: NonNull<c::JSContext>,
    v: c::JSValue,
    mut f: F,
) -> Result<(), String> {
    unsafe {
        let len_value = c::JS_GetPropertyStr(ctx, v, b"length\0".as_ptr() as *const c_char);
        let len = c::JS_GetInt(len_value) as u32;
        c::JS_FreeValue(ctx, len_value);
        for i in 0..len {
            let value = c::JS_GetPropertyUint32(ctx, v, i);
            f(value)?;
            c::JS_FreeValue(ctx, value);
        }
    }
    Ok(())
}

pub fn js_object_get_field_or_default<T: DecodeFromJSValue + Default>(
    ctx: NonNull<c::JSContext>,
    v: c::JSValue,
    field: &str,
) -> Result<T, String> {
    let value = js_object_get_option_field(ctx, v, field)?;
    Ok(value.unwrap_or_default())
}

pub fn js_val_into<T: DecodeFromJSValue>(
    ctx: *mut c::JSContext,
    v: c::JSValue,
) -> Result<T, &'static str> {
    DecodeFromJSValue::decode_into(ctx, v)
}

impl DecodeFromJSValue for u128 {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        unsafe {
            if c::JS_IsNumber(v) != 0 {
                let mut val = 0;
                if c::JS_ToInt64(ctx, &mut val, v) != 0 {
                    return Err("failed to decode u128")?;
                }
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
            let val = str.parse::<u128>().or(Err("invalid number or overflow"))?;

            Ok(val)
        }
    }
}

impl DecodeFromJSValue for u32 {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        unsafe {
            let mut v64 = 0;
            if c::JS_ToInt64Ext(ctx, &mut v64, v) != 0 {
                return Err("failed to decode u32");
            }
            v64.try_into().or(Err("number too large"))
        }
    }
}

impl DecodeFromJSValue for u64 {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        let v128 = u128::decode(ctx, v)?;
        v128.try_into().or(Err("number too large"))
    }
}

impl DecodeFromJSValue for bool {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        let b = unsafe { c::JS_ToBool(ctx, v) };
        if b < 0 {
            return Err("except bool");
        }
        Ok(b != 0)
    }
}

impl DecodeFromJSValue for Vec<u8> {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        unsafe {
            if c::JS_IsString(v) != 0 {
                let pstr = c::JS_ToCString(ctx, v);
                if pstr.is_null() {
                    return Err("invalid string");
                }
                let cstr = CStr::from_ptr(pstr);
                let str = cstr.to_string_lossy();
                let ret = match hex::decode(str.strip_prefix("0x").unwrap_or(&str)) {
                    Ok(bytes) => Ok(bytes),
                    Err(_) => Err("failed to decode bytes in hex string"),
                };
                c::JS_FreeCString(ctx, pstr);
                ret
            } else {
                let mut size = 0;
                let ptr = c::JS_Uint8ArrayGetBuffer(v, &mut size);
                if ptr.is_null() {
                    return Err("not a Uint8Array");
                }
                let buf = core::slice::from_raw_parts(ptr, size);
                Ok(buf.to_vec())
            }
        }
    }
}

impl<const N: usize> DecodeFromJSValue for [u8; N] {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        let bytes: Vec<u8> = DecodeFromJSValue::decode(ctx, v)?;
        Ok(bytes.try_into().or(Err("bytes length mismatch"))?)
    }
}

impl DecodeFromJSValue for String {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        unsafe {
            if c::JS_IsString(v) == 0 {
                return Err("expect a string");
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
}

impl DecodeFromJSValue for BTreeMap<String, String> {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        let JsValue::Object(value) = deserialize_value(ctx, &v)? else {
            return Err("decoding map requires a js object");
        };
        let me = value
            .into_iter()
            .flat_map(|(k, v)| {
                let JsValue::String(v) = v else {
                    return None;
                };
                Some((k, v))
            })
            .collect();
        Ok(me)
    }
}

pub struct HashableBytes(pub Vec<u8>);
impl DecodeFromJSValue for HashableBytes {
    fn decode(ctx: *mut c::JSContext, v: c::JSValue) -> Result<Self, &'static str> {
        let bytes = unsafe {
            if c::JS_IsString(v) != 0 {
                <String as DecodeFromJSValue>::decode(ctx, v)?.into_bytes()
            } else {
                <Vec<u8> as DecodeFromJSValue>::decode(ctx, v)?
            }
        };
        Ok(HashableBytes(bytes))
    }
}

pub fn js_val_from_bytes(ctx: *mut c::JSContext, bytes: &[u8]) -> c::JSValue {
    return unsafe { c::JS_NewUint8Array(ctx, bytes.as_ptr() as _, bytes.len() as _) };
}

fn js_create_bigint_function(context: *mut c::JSContext) -> c::JSValue {
    let global = unsafe { c::JS_GetGlobalObject(context) };
    assert_eq!(tag(global), TAG_OBJECT);

    let bigint_function = unsafe {
        c::JS_GetPropertyStr(
            context,
            global,
            core::ffi::CStr::from_bytes_with_nul(b"BigInt\0")
                .unwrap()
                .as_ptr(),
        )
    };
    assert_eq!(tag(bigint_function), TAG_OBJECT);
    unsafe { c::JS_FreeValue(context, global) };
    bigint_function
}

/// Serialize a Rust value into a quickjs runtime value.
pub fn serialize_value(
    context: *mut c::JSContext,
    value: JsValue,
) -> Result<c::JSValue, &'static str> {
    let v = match value {
        JsValue::Undefined => c::JS_UNDEFINED,
        JsValue::Null => c::JS_NULL,
        JsValue::Bool(flag) => {
            if flag {
                c::JS_TRUE
            } else {
                c::JS_FALSE
            }
        }
        JsValue::Int(val) => c::JS_MKVAL(c::JS_TAG_INT, val),
        JsValue::Float(val) => c::JS_MakeFloat64(val),
        JsValue::String(val) => {
            let qval = unsafe {
                c::JS_NewStringLen(context, val.as_ptr() as *const c_char, val.len() as _)
            };

            if tag(qval) == TAG_EXCEPTION {
                return Err("Could not create string in runtime");
            }

            qval
        }
        JsValue::Array(values) => {
            // Allocate a new array in the runtime.
            let arr = unsafe { c::JS_NewArray(context) };
            if tag(arr) == TAG_EXCEPTION {
                return Err("Could not create array in runtime");
            }

            for (index, value) in values.into_iter().enumerate() {
                let qvalue = match serialize_value(context, value) {
                    Ok(qval) => qval,
                    Err(e) => {
                        // Make sure to free the array if a individual element
                        // fails.

                        unsafe {
                            c::JS_FreeValue(context, arr);
                        }

                        return Err(e);
                    }
                };

                let ret = unsafe {
                    c::JS_DefinePropertyValueUint32(
                        context,
                        arr,
                        index as u32,
                        qvalue,
                        c::JS_PROP_C_W_E as i32,
                    )
                };
                if ret < 0 {
                    // Make sure to free the array if a individual
                    // element fails.
                    unsafe {
                        c::JS_FreeValue(context, arr);
                    }
                    return Err("Could not append element to array");
                }
            }
            arr
        }
        JsValue::Object(map) => {
            let obj = unsafe { c::JS_NewObject(context) };
            if tag(obj) == TAG_EXCEPTION {
                return Err("Could not create object");
            }

            for (key, value) in map {
                let ckey = alloc::ffi::CString::new(key).or(Err("invalid string"))?;

                let qvalue = serialize_value(context, value).map_err(|e| {
                    // Free the object if a property failed.
                    unsafe {
                        c::JS_FreeValue(context, obj);
                    }
                    e
                })?;

                let ret = unsafe {
                    c::JS_DefinePropertyValueStr(
                        context,
                        obj,
                        ckey.as_ptr(),
                        qvalue,
                        c::JS_PROP_C_W_E as i32,
                    )
                };
                if ret < 0 {
                    // Free the object if a property failed.
                    unsafe {
                        c::JS_FreeValue(context, obj);
                    }
                    return Err("Could not add add property to object");
                }
            }

            obj
        }
        JsValue::BigInt(bigint_string) => {
            let s = unsafe {
                c::JS_NewStringLen(
                    context,
                    bigint_string.as_ptr() as *const c_char,
                    bigint_string.len() as usize,
                )
            };
            scopeguard::defer! { unsafe {
                c::JS_FreeValue(context, s);
            }};

            if tag(s) != TAG_STRING {
                return Err("Could not construct String object needed to create BigInt object");
            }

            let mut args = alloc::vec![s];

            let bigint_function = js_create_bigint_function(context);
            scopeguard::defer! { unsafe {
                c::JS_FreeValue(context, bigint_function);
            }};
            let js_bigint =
                unsafe { c::JS_Call(context, bigint_function, c::JS_NULL, 1, args.as_mut_ptr()) };

            if tag(js_bigint) != TAG_BIG_INT {
                return Err("Could not construct BigInt object");
            }

            js_bigint
        }
        JsValue::Bytes(bytes) => unsafe {
            c::JS_NewUint8Array(context, bytes.as_ptr(), bytes.len() as _)
        },
        JsValue::Raw(value) => value,
    };
    Ok(v)
}

fn deserialize_array(
    context: *mut c::JSContext,
    raw_value: &c::JSValue,
) -> Result<JsValue, &'static str> {
    assert_eq!(tag(*raw_value), TAG_OBJECT);

    let length_name = b"length\0";

    let len_raw = unsafe { c::JS_GetPropertyStr(context, *raw_value, length_name.as_ptr() as _) };

    let len_res = deserialize_value(context, &len_raw);
    unsafe { c::JS_FreeValue(context, len_raw) };
    let len = match len_res? {
        JsValue::Int(x) => x,
        _ => {
            return Err("Could not determine array length");
        }
    };

    let mut values = Vec::new();
    for index in 0..(len as usize) {
        let value_raw = unsafe { c::JS_GetPropertyUint32(context, *raw_value, index as u32) };
        if tag(value_raw) == TAG_EXCEPTION {
            return Err("Could not build array");
        }
        let value_res = deserialize_value(context, &value_raw);
        unsafe { c::JS_FreeValue(context, value_raw) };

        let value = value_res?;
        values.push(value);
    }

    Ok(JsValue::Array(values))
}

fn deserialize_bytes(
    _ctx: *mut c::JSContext,
    raw_value: &c::JSValue,
) -> Result<JsValue, &'static str> {
    assert_eq!(tag(*raw_value), TAG_OBJECT);

    let mut len = 0;
    let ptr = unsafe { c::JS_Uint8ArrayGetBuffer(*raw_value, &mut len) };
    if ptr.is_null() {
        return Err("except Uint8Array");
    }
    let bytes = unsafe { &*core::ptr::slice_from_raw_parts(ptr, len) };
    Ok(JsValue::Bytes(bytes.to_vec()))
}

fn deserialize_object(
    context: *mut c::JSContext,
    obj: &c::JSValue,
) -> Result<JsValue, &'static str> {
    assert_eq!(tag(*obj), TAG_OBJECT);

    let mut properties: *mut c::JSPropertyEnum = core::ptr::null_mut();
    let mut count: u32 = 0;

    let flags = (c::JS_GPN_STRING_MASK | c::JS_GPN_SYMBOL_MASK | c::JS_GPN_ENUM_ONLY) as i32;
    let ret =
        unsafe { c::JS_GetOwnPropertyNames(context, &mut properties, &mut count, *obj, flags) };
    if ret != 0 {
        return Err("Could not get object properties");
    }

    scopeguard::defer! {
        for index in 0..count {
            let prop = unsafe { properties.offset(index as isize) };
            unsafe {
                c::JS_FreeAtom(context, (*prop).atom);
            }
        }
        unsafe {
            c::js_free(context, properties as *mut core::ffi::c_void);
        }
    }

    let mut map = BTreeMap::new();
    for index in 0..count {
        let prop = unsafe { properties.offset(index as isize) };
        let raw_value = unsafe { c::JS_GetPropertyInternal(context, *obj, (*prop).atom, *obj, 0) };
        if tag(raw_value) == TAG_EXCEPTION {
            return Err("Could not get object property");
        }

        let value_res = deserialize_value(context, &raw_value);
        unsafe {
            c::JS_FreeValue(context, raw_value);
        }
        let value = value_res?;

        let key_value = unsafe { c::JS_AtomToString(context, (*prop).atom) };
        if tag(key_value) == TAG_EXCEPTION {
            return Err("Could not get object property name");
        }

        let key_res = deserialize_value(context, &key_value);
        unsafe {
            c::JS_FreeValue(context, key_value);
        }
        let key = match key_res? {
            JsValue::String(s) => s,
            _ => {
                return Err("Could not get property name");
            }
        };
        map.insert(key, value);
    }

    Ok(JsValue::Object(map))
}

pub fn deserialize_value(
    context: *mut c::JSContext,
    value: &c::JSValue,
) -> Result<JsValue, &'static str> {
    let r = value;

    match tag(*r) {
        // Int.
        TAG_INT => {
            let val = c::JS_GetInt(*r);
            Ok(JsValue::Int(val))
        }
        // Bool.
        TAG_BOOL => {
            let raw = c::JS_GetBool(*r);
            let val = raw > 0;
            Ok(JsValue::Bool(val))
        }
        // Null.
        TAG_NULL => Ok(JsValue::Null),
        // Undefined.
        TAG_UNDEFINED => Ok(JsValue::Undefined),
        // Float.
        TAG_FLOAT64 => {
            let val = c::JS_GetFloat64(*r);
            Ok(JsValue::Float(val))
        }
        // String.
        TAG_STRING => {
            let ptr = unsafe { c::JS_ToCStringLen2(context, core::ptr::null_mut(), *r, 0) };

            if ptr.is_null() {
                return Err("Could not convert string: got a null pointer");
            }

            let cstr = unsafe { core::ffi::CStr::from_ptr(ptr) };

            let s = cstr.to_str().or(Err("invalid string"))?.to_string();

            // Free the c string.
            unsafe { c::JS_FreeCString(context, ptr) };

            Ok(JsValue::String(s))
        }
        // Object.
        TAG_OBJECT => {
            let is_array = unsafe { c::JS_IsArray(context, *r) } > 0;
            if is_array {
                deserialize_array(context, r)
            } else if unsafe { c::JS_IsUint8Array(*r) } > 0 {
                deserialize_bytes(context, r)
            } else {
                deserialize_object(context, r)
            }
        }
        // BigInt
        TAG_BIG_INT => {
            let ptr = unsafe { c::JS_ToCStringLen2(context, core::ptr::null_mut(), *r, 0) };

            if ptr.is_null() {
                return Err("Could not convert BigInt to string: got a null pointer");
            }

            let cstr = unsafe { core::ffi::CStr::from_ptr(ptr) };
            let bigint = cstr.to_string_lossy().to_string();

            // Free the c string.
            unsafe { c::JS_FreeCString(context, ptr) };

            Ok(JsValue::BigInt(bigint))
        }
        _ => Err("Unhandled JS_TAG"),
    }
}

fn tag(v: c::JSValue) -> i32 {
    c::JS_GetTag(v)
}
