use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use super::{c, Error, Result};

pub type Ptr<T> = Option<core::ptr::NonNull<T>>;

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

pub struct Value {
    value: c::JSValue,
    ctx: Ptr<c::JSContext>,
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

pub struct PairIter(Value);
impl Iterator for PairIter {
    type Item = Result<(Value, Value)>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = opt_try!(self.0.next())?;
        let key = opt_try!(next.get_property("0"));
        let value = opt_try!(next.get_property("1"));
        Some(Ok((key, value)))
    }
}
impl From<Value> for PairIter {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        let Ok(ctx) = self.context() else {
            return;
        };
        unsafe {
            c::JS_FreeValue(ctx, self.value);
            c::JS_FreeContext(ctx);
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self.context() {
            Ok(ctx) => Self {
                ctx: core::ptr::NonNull::new(unsafe { c::JS_DupContext(ctx) }),
                value: unsafe { c::JS_DupValue(ctx, self.value) },
            },
            Err(_) => Self {
                ctx: self.ctx,
                value: self.value,
            },
        }
    }
}

impl Value {
    pub fn new_cloned(ctx: *mut c::JSContext, value: c::JSValue) -> Self {
        Self::new_moved(ctx, unsafe { c::JS_DupValue(ctx, value) })
    }

    pub fn new_moved(ctx_ref: *mut c::JSContext, value: c::JSValue) -> Self {
        let ctx = core::ptr::NonNull::new(ctx_ref);
        if let Some(ctx) = ctx {
            unsafe { c::JS_DupContext(ctx.as_ptr()) };
        };
        Self { ctx, value }
    }

    pub fn into_raw(self) -> c::JSValue {
        match self.context() {
            Ok(ctx) => unsafe { c::JS_DupValue(ctx, self.value) },
            Err(_) => self.value,
        }
    }

    pub fn raw_value(&self) -> &c::JSValue {
        &self.value
    }

    pub fn context(&self) -> Result<*mut c::JSContext> {
        Ok(self.ctx.ok_or(Error::Static("context is null"))?.as_ptr())
    }

    pub fn get_property(&self, name: &str) -> Result<Self> {
        let ctx = self.context()?;
        let mut name_buf: tinyvec::TinyVec<[u8; 32]> = name.bytes().collect();
        name_buf.push(0);
        let value = unsafe { c::JS_GetPropertyStr(ctx, self.value, name_buf.as_ptr() as _) };
        let value = Self::new_moved(ctx, value);
        if value.is_exception() {
            Err(Error::JsException)
        } else {
            Ok(value)
        }
    }

    pub fn length(&self) -> Result<usize> {
        self.get_property("length")?
            .decode_i64()?
            .try_into()
            .or(Err(Error::Expect("usize")))
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
        let mut args: tinyvec::TinyVec<[_; 16]> = args.iter().map(|v| RawValue(v.value)).collect();
        let value = unsafe {
            c::JS_Call(
                ctx,
                self.value,
                this.value,
                args.len() as _,
                args.as_mut_ptr() as _,
            )
        };
        let ret = Self::new_moved(ctx, value);
        if ret.is_exception() {
            Err(Error::JsException)
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
        #[allow(non_snake_case)]
        let Object = get_global(self.context()?).get_property("Object")?;
        let entries_fn = Object.get_property("entries")?;
        let null = Value::null();
        let arr = entries_fn.call(&null, &[self.clone()])?;
        arr.call_method_if_exists("values", &[]).map(Into::into)
    }

    fn to_string_utf8(&self) -> Option<Utf8Repr> {
        let mut len: c::size_t = 0;
        let ptr = unsafe { c::JS_ToCStringLen(self.context().ok()?, &mut len, self.value) };
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
            c::JS_FreeCString(ctx, self.ptr as _);
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
    pub fn is_generic_object(&self) -> bool {
        unsafe { c::JS_IsGenericObject(self.value) != 0 }
    }
    pub fn is_function(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsFunction(ctx, self.value) != 0 }
    }
    pub fn is_constructor(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsConstructor(ctx, self.value) != 0 }
    }
    pub fn is_array(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsArray(ctx, self.value) != 0 }
    }
    pub fn is_error(&self) -> bool {
        let Ok(ctx) = self.context() else {
            return false;
        };
        unsafe { c::JS_IsError(ctx, self.value) != 0 }
    }
    pub fn is_uint8_array(&self) -> bool {
        unsafe { c::JS_IsUint8Array(self.value) != 0 }
    }
}

impl Value {
    pub const fn undefined() -> Self {
        Self {
            ctx: None,
            value: c::JS_UNDEFINED,
        }
    }
    pub const fn null() -> Self {
        Self {
            ctx: None,
            value: c::JS_NULL,
        }
    }
    pub fn from_bool(ctx: *mut c::JSContext, val: bool) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewBool(ctx, val as _)) }
    }
    pub fn from_i8(ctx: *mut c::JSContext, val: i8) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_u8(ctx: *mut c::JSContext, val: u8) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_i16(ctx: *mut c::JSContext, val: i16) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_u16(ctx: *mut c::JSContext, val: u16) -> Self {
        Self::from_i32(ctx, val as _)
    }
    pub fn from_i32(ctx: *mut c::JSContext, val: i32) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewInt32(ctx, val)) }
    }
    pub fn from_u32(ctx: *mut c::JSContext, val: u32) -> Self {
        Self::from_u128(ctx, val as _)
    }
    pub fn from_i64(ctx: *mut c::JSContext, val: i64) -> Self {
        Self::bigint(ctx, val)
    }
    pub fn from_u64(ctx: *mut c::JSContext, val: u64) -> Self {
        Self::biguint(ctx, val)
    }
    pub fn from_i128(ctx: *mut c::JSContext, val: i128) -> Self {
        Self::bigint_from_str(ctx, &val.to_string()).expect("Failed to create BigInt fron i128")
    }
    pub fn from_u128(ctx: *mut c::JSContext, val: u128) -> Self {
        Self::bigint_from_str(ctx, &val.to_string()).expect("Failed to create BigInt fron i128")
    }
    pub fn from_f32(ctx: *mut c::JSContext, val: f32) -> Self {
        Self::from_f64(ctx, val as _)
    }
    pub fn from_f64(ctx: *mut c::JSContext, val: f64) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewFloat64(ctx, val)) }
    }
    pub fn bigint(ctx: *mut c::JSContext, val: i64) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewBigInt64(ctx, val)) }
    }
    pub fn bigint_from_str(ctx: *mut c::JSContext, val: &str) -> Result<Self> {
        let val = Self::from_str(ctx, val);
        get_global(ctx).call_method("BigInt", &[val])
    }
    pub fn biguint(ctx: *mut c::JSContext, val: u64) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewBigUint64(ctx, val)) }
    }
    pub fn from_str(ctx: *mut c::JSContext, val: &str) -> Self {
        unsafe {
            let val = c::JS_NewStringLen(ctx, val.as_ptr() as _, val.len() as _);
            Self::new_moved(ctx, val)
        }
    }
    pub fn from_bytes(ctx: *mut c::JSContext, bytes: &[u8]) -> Self {
        unsafe {
            Self::new_moved(
                ctx,
                c::JS_NewUint8Array(ctx, bytes.as_ptr() as _, bytes.len() as _),
            )
        }
    }
    pub fn new_array(ctx: *mut c::JSContext) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewArray(ctx)) }
    }
    pub fn new_object(ctx: *mut c::JSContext) -> Self {
        unsafe { Self::new_moved(ctx, c::JS_NewObject(ctx)) }
    }
}

impl Value {
    pub fn set_property(&self, key: &str, value: &Value) -> Result<(), Error> {
        let ctx = self.context()?;
        unsafe {
            let key = c::JS_NewAtomLen(ctx, key.as_ptr() as _, key.len() as _);
            let r = c::JS_SetProperty(ctx, self.value, key, c::JS_DupValue(ctx, value.value));
            c::JS_FreeAtom(ctx, key);
            if r != 0 {
                Ok(())
            } else {
                Err(Error::Custom(format!("Failed to set property: {key}")))
            }
        }
    }
    pub fn set_property_fn(&self, key: &str, f: JsCFunction) -> Result<(), Error> {
        let ctx = self.context()?;
        unsafe {
            let c_name = alloc::ffi::CString::new(key).or(Err(Error::Expect("function name")))?;
            c::JS_SetPropertyStr(
                ctx,
                self.value,
                c_name.as_ptr(),
                c::JS_NewCFunction(ctx, Some(f), c_name.as_ptr(), 0),
            );
        }
        Ok(())
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
            Ok(unsafe { c::JS_ToBool(self.context()?, self.value) != 0 })
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
                let r = c::JS_ToInt64Ext(self.context()?, &mut v, self.value);
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
                    c::JS_GetArrayBuffer(ctx, &mut len, self.value)
                } else {
                    c::JS_Uint8ArrayGetBuffer(self.value, &mut len)
                }
            };
            if ptr.is_null() {
                return Err(Error::Expect("bytes"));
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
            let s = self.to_string_utf8().ok_or(Error::Expect("invalid utf8"))?;
            let s = s.as_str();
            if !(s.starts_with("0x") || s.starts_with("0X")) {
                return Err(Error::Expect("hex starts with 0x"));
            }
            hex::decode(&s[2..]).or(Err(Error::Expect("invalid hex")))
        } else {
            Err(Error::Expect("bytes"))
        }
    }
    pub fn parse<T: core::str::FromStr>(&self) -> Option<T> {
        self.to_string_utf8()?.as_str().parse::<T>().ok()
    }
}

pub fn get_global(context: *mut c::JSContext) -> Value {
    Value::new_moved(context, unsafe { c::JS_GetGlobalObject(context) })
}
