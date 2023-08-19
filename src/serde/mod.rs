use core::f32::consts::E;
use core::fmt::Display;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::c;

mod de;
mod ser;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Static(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Custom(s) => f.write_str(s),
            Error::Static(s) => f.write_str(s),
        }
    }
}

impl serde::de::StdError for Error {
    fn source(&self) -> Option<&(dyn serde::de::StdError + 'static)> {
        None
    }
}

impl serde::de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }

    #[cold]
    fn invalid_type(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        if let serde::de::Unexpected::Unit = unexp {
            Error::Custom(alloc::format!("invalid type: null, expected {}", exp))
        } else {
            Error::Custom(alloc::format!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}

impl serde::ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

pub struct Value {
    value: c::JSValue,
    ctx: *mut c::JSContext,
}

pub struct Iter(Value);

impl Iterator for Iter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl From<Value> for Iter {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

pub struct PairIter(Value);
impl Iterator for PairIter {
    type Item = (Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next()?;
        let key = next.get_property("0");
        let value = next.get_property("1");
        Some((key, value))
    }
}
impl From<Value> for PairIter {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            c::JS_FreeValue(self.ctx, self.value);
            c::JS_FreeContext(self.ctx);
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Self::new_cloned(self.ctx, self.value)
    }
}

impl Value {
    pub fn new_cloned(ctx: *mut c::JSContext, value: c::JSValue) -> Self {
        Self::new_moved(ctx, unsafe { c::JS_DupValue(ctx, value) })
    }

    pub fn new_moved(ctx_ref: *mut c::JSContext, value: c::JSValue) -> Self {
        Self {
            ctx: unsafe { c::JS_DupContext(ctx_ref) },
            value,
        }
    }

    pub fn get_property(&self, name: &str) -> Self {
        let mut name_buf: tinyvec::TinyVec<[u8; 32]> = name.bytes().collect();
        name_buf.push(0);
        unsafe {
            let value = c::JS_GetPropertyStr(self.ctx, self.value, name_buf.as_ptr() as _);
            Self::new_moved(self.ctx, value)
        }
    }

    pub fn length(&self) -> Option<usize> {
        self.get_property("length").decode_i64()?.try_into().ok()
    }

    pub fn next(&self) -> Option<Self> {
        let next_fn = self.get_property("next");
        if next_fn.is_null() {
            None
        } else {
            let next_val = next_fn.call(self, &[]);
            if next_val.is_undefined() || next_val.is_exception() {
                None
            } else {
                Some(next_val)
            }
        }
    }

    pub fn call_method(&self, name: &str, args: &[Value]) -> Self {
        let method = self.get_property(name);
        method.call(self, args)
    }

    pub fn call_method_if_exists(&self, name: &str, args: &[Value]) -> Option<Self> {
        let method = self.get_property(name);
        if !method.is_function() {
            return None;
        }
        Some(method.call(self, args))
    }

    pub fn call(&self, this: &Value, args: &[Value]) -> Self {
        #[repr(transparent)]
        struct V(c::JSValue);
        impl Default for V {
            fn default() -> Self {
                Self(c::JS_UNDEFINED)
            }
        }
        let mut args: tinyvec::TinyVec<[_; 16]> = args.iter().map(|v| V(v.value)).collect();
        unsafe {
            let value = c::JS_Call(
                self.ctx,
                self.value,
                this.value,
                args.len() as _,
                args.as_mut_ptr() as _,
            );
            Self::new_moved(self.ctx, value)
        }
    }

    pub fn values(&self) -> Option<Iter> {
        self.call_method_if_exists("values", &[]).map(Into::into)
    }

    pub fn keys(&self) -> Option<Iter> {
        self.call_method_if_exists("keys", &[]).map(Into::into)
    }

    pub fn entries(&self) -> Option<PairIter> {
        self.call_method_if_exists("entries", &[]).map(Into::into)
    }

    fn to_string_utf8(&self) -> Option<Utf8Repr> {
        let mut len: c::size_t = 0;
        let ptr = unsafe { c::JS_ToCStringLen(self.ctx, &mut len, self.value) };
        if ptr.is_null() {
            return None;
        }
        Some(Utf8Repr {
            value: self,
            ptr,
            len,
        })
    }
}

struct Utf8Repr<'a> {
    value: &'a Value,
    ptr: *const i8,
    len: usize,
}

impl Drop for Utf8Repr<'_> {
    fn drop(&mut self) {
        unsafe {
            c::JS_FreeCString(self.value.ctx, self.ptr as _);
        }
    }
}

impl Utf8Repr<'_> {
    fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.ptr as _, self.len))
        }
    }
}

impl Value {
    pub fn is_exception(&self) -> bool {
        unsafe { c::JS_IsException(self.value) != 0 }
    }
    pub fn is_undefined(&self) -> bool {
        unsafe { c::JS_IsUndefined(self.value) != 0 }
    }
    pub fn is_null(&self) -> bool {
        unsafe { c::JS_IsNull(self.value) != 0 }
    }
    pub fn is_bool(&self) -> bool {
        unsafe { c::JS_IsBool(self.value) != 0 }
    }
    pub fn is_number(&self) -> bool {
        unsafe { c::JS_IsNumber(self.value) != 0 }
    }
    pub fn is_big_int(&self) -> bool {
        unsafe { c::JS_IsBigInt(self.value) != 0 }
    }
    pub fn is_string(&self) -> bool {
        unsafe { c::JS_IsString(self.value) != 0 }
    }
    pub fn is_symbol(&self) -> bool {
        unsafe { c::JS_IsSymbol(self.value) != 0 }
    }
    pub fn is_object(&self) -> bool {
        unsafe { c::JS_IsObject(self.value) != 0 }
    }
    pub fn is_function(&self) -> bool {
        unsafe { c::JS_IsFunction(self.ctx, self.value) != 0 }
    }
    pub fn is_constructor(&self) -> bool {
        unsafe { c::JS_IsConstructor(self.ctx, self.value) != 0 }
    }
    pub fn is_array(&self) -> bool {
        unsafe { c::JS_IsArray(self.ctx, self.value) != 0 }
    }
    pub fn is_error(&self) -> bool {
        unsafe { c::JS_IsError(self.ctx, self.value) != 0 }
    }
    pub fn is_uint8_array(&self) -> bool {
        unsafe { c::JS_IsUint8Array(self.value) != 0 }
    }
}

impl Value {
    pub fn decode_bool(&self) -> Option<bool> {
        if self.is_bool() {
            Some(unsafe { c::JS_ToBool(self.ctx, self.value) != 0 })
        } else {
            None
        }
    }
    pub fn decode_string(&self) -> Option<String> {
        if self.is_string() {
            Some(self.to_string_utf8()?.as_str().into())
        } else {
            None
        }
    }
    pub fn decode_i64(&self) -> Option<i64> {
        if self.is_number() || self.is_big_int() {
            let mut v = 0;
            unsafe {
                let r = c::JS_ToInt64Ext(self.ctx, &mut v, self.value);
                if r == 0 {
                    return Some(v);
                }
            }
        }
        None
    }
    pub fn decode_i8(&self) -> Option<i8> {
        self.decode_i64()?.try_into().ok()
    }
    pub fn decode_u8(&self) -> Option<u8> {
        self.decode_i64()?.try_into().ok()
    }
    pub fn decode_i32(&self) -> Option<i32> {
        self.decode_i64()?.try_into().ok()
    }
    pub fn decode_u32(&self) -> Option<u32> {
        self.decode_i64()?.try_into().ok()
    }
    pub fn decode_number<N: core::str::FromStr>(&self) -> Option<N> {
        // TODO: optimize performance
        if self.is_number() || self.is_big_int() {
            self.parse()
        } else {
            None
        }
    }
    pub fn decode_bytes(&self) -> Option<Vec<u8>> {
        if self.is_uint8_array() || self.is_uint8_array() {
            let mut len: c::size_t = 0;
            let ptr = unsafe {
                if self.is_uint8_array() {
                    c::JS_GetArrayBuffer(self.ctx, &mut len, self.value)
                } else {
                    c::JS_Uint8ArrayGetBuffer(self.value, &mut len)
                }
            };
            if ptr.is_null() {
                return None;
            }
            let mut v = Vec::with_capacity(len);
            unsafe {
                core::ptr::copy_nonoverlapping(ptr as _, v.as_mut_ptr(), len);
                v.set_len(len);
            }
            Some(v)
        } else if self.is_array() {
            let len = self.length()?;
            let mut v = Vec::with_capacity(len);
            for i in 0..len {
                let v2 = self.get_property(&i.to_string());
                if v2.is_undefined() {
                    return None;
                }
                v.push(v2.decode_u8()?);
            }
            Some(v)
        } else if self.is_string() {
            let s = self.to_string_utf8()?;
            let s = s.as_str();
            if !(s.starts_with("0x") || s.starts_with("0X")) {
                return None;
            }
            hex::decode(&s[2..]).ok()
        } else {
            None
        }
    }
    pub fn parse<T: core::str::FromStr>(&self) -> Option<T> {
        self.to_string_utf8()?.as_str().parse::<T>().ok()
    }
}
