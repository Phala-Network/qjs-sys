use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate as js;
use crate::{
    opaque_value::{new_opaque_object, opaque_object_get_data, opaque_object_take_data},
    FromJsValue,
};

use super::{c, Error, Result};

type JsCFunction = unsafe extern "C" fn(
    ctx: *mut c::JSContext,
    this_val: c::JSValueConst,
    argc: core::ffi::c_int,
    argv: *mut c::JSValue,
) -> c::JSValue;

#[repr(transparent)]
pub struct RawValue(pub c::JSValue);
impl Default for RawValue {
    fn default() -> Self {
        Self(c::JS_UNDEFINED)
    }
}

pub use JsValue as Value;
pub enum JsValue {
    Undefined,
    Null,
    Exception,
    Other { value: c::JSValue, ctx: js::Context },
}

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_exception() {
            write!(f, "<JS exception>")
        } else if self.is_undefined() {
            write!(f, "<JS undefined>")
        } else if self.is_null() {
            write!(f, "<JS null>")
        } else {
            write!(f, "<JS value>")
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::undefined()
    }
}

pub struct Iter(Value);

impl Iterator for Iter {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().transpose()
    }
}

impl From<Value> for Iter {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.to_string_utf8() {
            Some(s) => s.as_str().fmt(f),
            None => write!(f, "<JS value>"),
        }
    }
}

pub struct PairIter {
    inner: Value,
    len: Option<usize>,
}
impl PairIter {
    pub fn new(inner: Value, len: Option<usize>) -> Self {
        Self { inner, len }
    }
    pub fn length(&self) -> Option<usize> {
        self.len
    }
}
impl Iterator for PairIter {
    type Item = Result<(Value, Value)>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = opt_try!(self.inner.next())?;
        let key = opt_try!(next.get_property("0"));
        let value = opt_try!(next.get_property("1"));
        Some(Ok((key, value)))
    }
}
impl From<Value> for PairIter {
    fn from(value: Value) -> Self {
        Self {
            inner: value,
            len: None,
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        let Ok(ctx) = self.context() else {
            return;
        };
        unsafe {
            c::JS_FreeValue(ctx.as_ptr(), *self.raw_value());
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Undefined => Self::Undefined,
            Self::Null => Self::Null,
            Self::Exception => Self::Exception,
            Self::Other { ctx, value } => Self::new_cloned(ctx, *value),
        }
    }
}

impl Value {
    pub fn new_cloned(ctx: &js::Context, value: c::JSValue) -> Self {
        Self::new_moved(ctx, unsafe { c::JS_DupValue(ctx.as_ptr(), value) })
    }

    pub fn new_moved(ctx: &js::Context, value: c::JSValue) -> Self {
        Self::Other {
            ctx: ctx.clone(),
            value,
        }
    }

    pub fn new_opaque_object<T: 'static>(ctx: &js::Context, value: T) -> Self {
        new_opaque_object(ctx, value)
    }

    pub fn opaque_object_data<T: 'static>(&self) -> Option<&T> {
        opaque_object_get_data(self)
    }

    pub fn opaque_object_take_data<T: 'static>(&self) -> Option<T> {
        Some(*opaque_object_take_data(self)?)
    }

    pub fn leak(self) -> c::JSValue {
        let value = *self.raw_value();
        match self.context() {
            Ok(ctx) => unsafe { c::JS_DupValue(ctx.as_ptr(), value) },
            Err(_) => value,
        }
    }

    pub fn raw_value(&self) -> &c::JSValue {
        match self {
            Self::Undefined => &c::JS_UNDEFINED,
            Self::Null => &c::JS_NULL,
            Self::Exception => &c::JS_EXCEPTION,
            Self::Other { value, .. } => value,
        }
    }

    #[track_caller]
    pub fn context(&self) -> Result<&js::Context> {
        match self {
            Self::Undefined => Err(Error::Static("no context for undefined")),
            Self::Null => Err(Error::Static("no context for null")),
            Self::Exception => Err(Error::Static("no context for exception")),
            Self::Other { ctx, .. } => Ok(ctx),
        }
    }

    pub fn index(&self, ind: usize) -> Result<Self> {
        self.get_property(&ind.to_string())
    }

    pub fn get_property(&self, name: &str) -> Result<Self> {
        let ctx = self.context()?;
        let mut name_buf: tinyvec::TinyVec<[u8; 32]> = name.bytes().collect();
        name_buf.push(0);
        let value = unsafe {
            c::JS_GetPropertyStr(ctx.as_ptr(), *self.raw_value(), name_buf.as_ptr() as _)
        };
        let value = Self::new_moved(ctx, value);
        if value.is_exception() {
            Err(Error::JsException(ctx.get_exception_str()))
        } else {
            Ok(value)
        }
    }

    pub fn get_property_t<T: FromJsValue>(&self, name: &str) -> Result<T> {
        T::from_js_value(self.get_property(name)?)
    }

    pub fn length(&self) -> Result<usize> {
        self.get_property_t("length")
    }

    pub fn next(&self) -> Result<Option<Self>> {
        let next_fn = self.get_property("next")?;
        if next_fn.is_null() {
            Err(Error::Expect("iterator"))
        } else {
            let next_val = next_fn.call(self, &[])?;
            let done = next_val.get_property("done")?;
            if done.decode_bool()? {
                Ok(None)
            } else {
                let value = next_val.get_property("value")?;
                Ok(Some(value))
            }
        }
    }

    pub fn call_method(&self, name: &str, args: &[Value]) -> Result<Self> {
        let method = self.get_property(name)?;
        method.call(self, args)
    }

    pub fn call_method_if_exists(&self, name: &str, args: &[Value]) -> Result<Self> {
        let method = self.get_property(name)?;
        if !method.is_function() {
            return Err(Error::Expect("function"));
        }
        method.call(self, args)
    }

    pub fn call(&self, this: &Value, args: &[Value]) -> Result<Self> {
        let ctx = self.context()?;
        let mut args: tinyvec::TinyVec<[_; 16]> =
            args.iter().map(|v| RawValue(*v.raw_value())).collect();
        let value = unsafe {
            c::JS_Call(
                ctx.as_ptr(),
                *self.raw_value(),
                *this.raw_value(),
                args.len() as _,
                args.as_mut_ptr() as _,
            )
        };
        let ret = Self::new_moved(ctx, value);
        if ret.is_exception() {
            Err(Error::JsException(ctx.get_exception_str()))
        } else {
            Ok(ret)
        }
    }

    pub fn values(&self) -> Result<Iter> {
        self.call_method_if_exists("values", &[]).map(Into::into)
    }

    pub fn keys(&self) -> Result<Iter> {
        self.call_method_if_exists("keys", &[]).map(Into::into)
    }

    pub fn entries(&self) -> Result<PairIter> {
        if self.is_undefined() || self.is_null() {
            return Err(Error::Expect("object for entries"));
        }
        #[allow(non_snake_case)]
        let Object = get_global(self.context()?).get_property("Object")?;
        let entries_fn = Object.get_property("entries")?;
        let null = Value::null();
        let arr = entries_fn.call(&null, &[self.clone()])?;
        let len = arr.length().ok();
        let iter = arr.call_method_if_exists("values", &[])?;
        Ok(PairIter::new(iter, len))
    }

    fn to_string_utf8(&self) -> Option<Utf8Repr> {
        let mut len: c::size_t = 0;
        let ptr = unsafe {
            c::JS_ToCStringLen(self.context().ok()?.as_ptr(), &mut len, *self.raw_value())
        };
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
        let Ok(ctx) = self.value.context() else {
            return;
        };
        unsafe {
            c::JS_FreeCString(ctx.as_ptr(), self.ptr as _);
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
        unsafe { c::JS_IsException(*self.raw_value()) != 0 }
    }
    pub fn is_undefined(&self) -> bool {
        unsafe { c::JS_IsUndefined(*self.raw_value()) != 0 }
    }
    pub fn is_null(&self) -> bool {
        unsafe { c::JS_IsNull(*self.raw_value()) != 0 }
    }
    pub fn is_bool(&self) -> bool {
        unsafe { c::JS_IsBool(*self.raw_value()) != 0 }
    }
    pub fn is_number(&self) -> bool {
        unsafe { c::JS_IsNumber(*self.raw_value()) != 0 }
    }
    pub fn is_big_int(&self) -> bool {
        unsafe { c::JS_IsBigInt(*self.raw_value()) != 0 }
    }
    pub fn is_string(&self) -> bool {
        unsafe { c::JS_IsString(*self.raw_value()) != 0 }
    }
    pub fn is_symbol(&self) -> bool {
        unsafe { c::JS_IsSymbol(*self.raw_value()) != 0 }
    }
    pub fn is_object(&self) -> bool {
        unsafe { c::JS_IsObject(*self.raw_value()) != 0 }
    }
    pub fn is_generic_object(&self) -> bool {
        unsafe { c::JS_IsGenericObject(*self.raw_value()) != 0 }
    }
    pub fn is_function(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsFunction(ctx.as_ptr(), *self.raw_value()) != 0 }
    }
    pub fn is_constructor(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsConstructor(ctx.as_ptr(), *self.raw_value()) != 0 }
    }
    pub fn is_array(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsArray(ctx.as_ptr(), *self.raw_value()) != 0 }
    }
    pub fn is_error(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsError(ctx.as_ptr(), *self.raw_value()) != 0 }
    }
    pub fn is_uint8_array(&self) -> bool {
        unsafe { c::JS_IsUint8Array(*self.raw_value()) != 0 }
    }
}

impl Value {
    pub const fn undefined() -> Self {
        Self::Undefined
    }
    pub const fn null() -> Self {
        Self::Null
    }
    pub fn from_bool(ctx: &js::Context, val: bool) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewBool(ctx.as_ptr(), val as _)) }
    }
    pub fn from_i8(ctx: &js::Context, val: i8) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_u8(ctx: &js::Context, val: u8) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_i16(ctx: &js::Context, val: i16) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_u16(ctx: &js::Context, val: u16) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_i32(ctx: &js::Context, val: i32) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewInt32(ctx.as_ptr(), val)) }
    }
    pub fn from_u32(ctx: &js::Context, val: u32) -> Self {
        if val <= i32::MAX as u32 {
            return Self::from_i32(ctx, val as _);
        }
        Self::from_u128(ctx, val as _)
    }
    pub fn from_i64(ctx: &js::Context, val: i64) -> Self {
        if val <= i32::MAX.into() {
            return Self::from_i32(ctx, val as _);
        }
        Self::bigint(ctx, val)
    }
    pub fn from_u64(ctx: &js::Context, val: u64) -> Self {
        if val <= i32::MAX as u64 {
            return Self::from_i32(ctx, val as _);
        }
        Self::biguint(ctx, val)
    }
    pub fn from_i128(ctx: &js::Context, val: i128) -> Self {
        Self::bigint_from_str(ctx, &val.to_string()).expect("Failed to create BigInt fron i128")
    }
    pub fn from_u128(ctx: &js::Context, val: u128) -> Self {
        Self::bigint_from_str(ctx, &val.to_string()).expect("Failed to create BigInt fron i128")
    }
    pub fn from_f32(ctx: &js::Context, val: f32) -> Self {
        Self::from_f64(ctx, val as _)
    }
    pub fn from_f64(ctx: &js::Context, val: f64) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewFloat64(ctx.as_ptr(), val)) }
    }
    pub fn from_usize(ctx: &js::Context, val: usize) -> Self {
        Self::from_u64(ctx, val as _)
    }
    pub fn bigint(ctx: &js::Context, val: i64) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewBigInt64(ctx.as_ptr(), val)) }
    }
    pub fn bigint_from_str(ctx: &js::Context, val: &str) -> Result<Self> {
        let val = Self::from_str(ctx, val);
        get_global(ctx).call_method("BigInt", &[val])
    }
    pub fn biguint(ctx: &js::Context, val: u64) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewBigUint64(ctx.as_ptr(), val)) }
    }
    pub fn from_str(ctx: &js::Context, val: &str) -> Self {
        unsafe {
            let val = c::JS_NewStringLen(ctx.as_ptr(), val.as_ptr() as _, val.len() as _);
            Self::new_moved(ctx, val)
        }
    }
    pub fn from_bytes(ctx: &js::Context, bytes: &[u8]) -> Self {
        unsafe {
            Self::new_moved(
                ctx,
                c::JS_NewUint8ArrayCopy(ctx.as_ptr(), bytes.as_ptr() as _, bytes.len() as _),
            )
        }
    }
    pub fn new_array(ctx: &js::Context) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewArray(ctx.as_ptr())) }
    }
    pub fn new_object(ctx: &js::Context) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewObject(ctx.as_ptr())) }
    }
}

impl Value {
    pub fn index_set(&self, ind: usize, value: &Value) -> Result<(), Error> {
        self.set_property(&ind.to_string(), value)
    }

    pub fn set_property(&self, key: &str, value: &Value) -> Result<(), Error> {
        let ctx = self.context()?;
        unsafe {
            let key = c::JS_NewAtomLen(ctx.as_ptr(), key.as_ptr() as _, key.len() as _);
            let r = c::JS_SetProperty(
                ctx.as_ptr(),
                *self.raw_value(),
                key,
                c::JS_DupValue(ctx.as_ptr(), *value.raw_value()),
            );
            c::JS_FreeAtom(ctx.as_ptr(), key);
            if r != 0 {
                Ok(())
            } else {
                Err(Error::Custom(format!("Failed to set property: {key}")))
            }
        }
    }
    pub fn set_prototype(&self, proto: &Value) -> Result<(), Error> {
        let ctx = self.context()?;
        unsafe {
            let r = c::JS_SetPrototype(ctx.as_ptr(), *self.raw_value(), *proto.raw_value());
            if r == 1 {
                Ok(())
            } else {
                Err(Error::Static("Failed to set prototype"))
            }
        }
    }
    pub fn define_property_fn(&self, key: &str, f: JsCFunction) -> Result<(), Error> {
        let ctx = self.context()?;
        let f = unsafe {
            c::JS_NewCFunctionLen(ctx.as_ptr(), Some(f), key.as_ptr() as _, key.len() as _, 0)
        };
        self.define_property_value(key, Value::new_moved(ctx, f))
    }
    pub fn define_property_value(&self, key: &str, f: Value) -> Result<(), Error> {
        unsafe {
            let ctx = self.context()?.as_ptr();
            let name = c::JS_NewAtomLen(ctx, key.as_ptr() as _, key.len() as _);
            let r = c::JS_DefinePropertyValue(
                ctx,
                *self.raw_value(),
                name,
                f.leak(),
                c::JS_PROP_C_W_E as _,
            );
            c::JS_FreeAtom(ctx, name);
            if r != 0 {
                Ok(())
            } else {
                Err(Error::Custom(format!("Failed to define property: {key}")))
            }
        }
    }
    pub fn array_push(&self, value: &Value) -> Result<()> {
        _ = self
            .call_method("push", &[value.clone()])
            .or(Err(Error::Static("Failed to push value to array")))?;
        Ok(())
    }
}

impl Value {
    pub fn decode_bool(&self) -> Result<bool> {
        if self.is_bool() {
            Ok(unsafe { c::JS_ToBool(self.context()?.as_ptr(), *self.raw_value()) != 0 })
        } else {
            Err(Error::Expect("bool"))
        }
    }
    pub fn decode_string(&self) -> Result<String> {
        if self.is_string() {
            Ok(self
                .to_string_utf8()
                .ok_or(Error::Expect("string"))?
                .as_str()
                .into())
        } else {
            Err(Error::Expect("string"))
        }
    }
    pub fn decode_i8(&self) -> Result<i8> {
        self.decode_i64()?.try_into().or(Err(Error::Expect("i8")))
    }
    pub fn decode_u8(&self) -> Result<u8> {
        self.decode_i64()?.try_into().or(Err(Error::Expect("u8")))
    }
    pub fn decode_i16(&self) -> Result<i16> {
        self.decode_i64()?.try_into().or(Err(Error::Expect("i16")))
    }
    pub fn decode_u16(&self) -> Result<u16> {
        self.decode_i64()?.try_into().or(Err(Error::Expect("u16")))
    }
    pub fn decode_i32(&self) -> Result<i32> {
        self.decode_i64()?.try_into().or(Err(Error::Expect("i32")))
    }
    pub fn decode_u32(&self) -> Result<u32> {
        self.decode_i64()?.try_into().or(Err(Error::Expect("u32")))
    }
    pub fn decode_i64(&self) -> Result<i64> {
        if self.is_number() || self.is_big_int() {
            let mut v = 0;
            unsafe {
                let r = c::JS_ToInt64Ext(self.context()?.as_ptr(), &mut v, *self.raw_value());
                if r == 0 {
                    return Ok(v);
                }
            }
        }
        Err(Error::Expect("i64"))
    }
    pub fn decode_u64(&self) -> Result<u64> {
        self.decode_number().or(Err(Error::Expect("u64")))
    }
    pub fn decode_usize(&self) -> Result<usize> {
        self.decode_u64()
            .or(Err(Error::Expect("usize")))?
            .try_into()
            .or(Err(Error::Expect("usize")))
    }
    pub fn decode_f32(&self) -> Result<f32> {
        self.decode_number().or(Err(Error::Expect("f32")))
    }
    pub fn decode_f64(&self) -> Result<f64> {
        self.decode_number().or(Err(Error::Expect("f64")))
    }
    pub fn decode_i128(&self) -> Result<i128> {
        self.decode_number().or(Err(Error::Expect("i128")))
    }
    pub fn decode_u128(&self) -> Result<u128> {
        self.decode_number().or(Err(Error::Expect("u128")))
    }
    pub fn decode_number<N: core::str::FromStr>(&self) -> Result<N> {
        // TODO: optimize performance
        if self.is_number() || self.is_big_int() {
            self.parse().ok_or(Error::Expect("number"))
        } else {
            Err(Error::Expect("number"))
        }
    }
    pub fn decode_bytes(&self) -> Result<Vec<u8>> {
        if self.is_uint8_array() || self.is_uint8_array() {
            let mut len: c::size_t = 0;
            let ctx = self.context()?;
            let ptr = unsafe {
                if self.is_uint8_array() {
                    c::JS_Uint8ArrayGetBuffer(*self.raw_value(), &mut len)
                } else {
                    c::JS_GetArrayBuffer(ctx.as_ptr(), &mut len, *self.raw_value())
                }
            };
            if ptr.is_null() {
                return Err(Error::Static("invalid bytes"));
            }
            let mut v = Vec::with_capacity(len);
            unsafe {
                core::ptr::copy_nonoverlapping(ptr as _, v.as_mut_ptr(), len);
                v.set_len(len);
            }
            Ok(v)
        } else if self.is_array() {
            let len = self.length()?;
            let mut v = Vec::with_capacity(len);
            for i in 0..len {
                let v2 = self.get_property(&i.to_string())?;
                v.push(v2.decode_u8()?);
            }
            Ok(v)
        } else if self.is_string() {
            #[cfg(feature = "treat-hex-as-bytes")]
            {
                self.decode_bytes_maybe_hex()
            }
            #[cfg(not(feature = "treat-hex-as-bytes"))]
            {
                let s = self.to_string_utf8().ok_or(Error::Expect("string"))?;
                Ok(s.as_str().as_bytes().to_vec())
            }
        } else {
            Err(Error::Expect("bytes-like value"))
        }
    }

    pub fn decode_bytes_maybe_hex(&self) -> Result<Vec<u8>> {
        if self.is_string() {
            let s = self.to_string_utf8().ok_or(Error::Expect("string"))?;
            let s = s.as_str();
            if s.starts_with("0x") || s.starts_with("0X") {
                let s = &s[2..];
                Ok(hex::decode(s).or(Err(Error::Expect("hex string")))?)
            } else {
                Ok(s.as_bytes().to_vec())
            }
        } else {
            self.decode_bytes()
        }
    }
    pub fn parse<T: core::str::FromStr>(&self) -> Option<T> {
        self.to_string_utf8()?.as_str().parse::<T>().ok()
    }
}

pub fn get_global(context: &js::Context) -> Value {
    Value::new_moved(context, unsafe { c::JS_GetGlobalObject(context.as_ptr()) })
}
