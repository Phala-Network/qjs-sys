use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use super::{Error, Value};
use crate::c;

type Result<T> = core::result::Result<T, Error>;

pub trait FromJsValue {
    fn from_js_value(js_value: Value) -> Result<Self>
    where
        Self: Sized;
}

impl FromJsValue for Value {
    fn from_js_value(js_value: Value) -> Result<Self> {
        Ok(js_value)
    }
}

macro_rules! impl_from_for {
    ($t: ident, $decode_fn: ident) => {
        impl FromJsValue for $t {
            fn from_js_value(js_value: Value) -> Result<Self> {
                js_value.$decode_fn().or(Err(Error::Expect(stringify!($t))))
            }
        }
    };
}

impl_from_for!(i8, decode_i8);
impl_from_for!(i16, decode_i16);
impl_from_for!(i32, decode_i32);
impl_from_for!(i64, decode_i64);
impl_from_for!(u8, decode_u8);
impl_from_for!(u16, decode_u16);
impl_from_for!(u32, decode_u32);
impl_from_for!(u64, decode_u64);
impl_from_for!(f32, decode_f32);
impl_from_for!(f64, decode_f64);
impl_from_for!(i128, decode_i128);
impl_from_for!(u128, decode_u128);
impl_from_for!(bool, decode_bool);
impl_from_for!(String, decode_string);

impl FromJsValue for () {
    fn from_js_value(js_value: Value) -> Result<Self> {
        if js_value.is_null() || js_value.is_undefined() {
            Ok(())
        } else {
            Err(Error::Expect("()"))
        }
    }
}

impl<T: FromJsValue> FromJsValue for Option<T> {
    fn from_js_value(js_value: Value) -> Result<Self> {
        if js_value.is_null() || js_value.is_undefined() {
            Ok(None)
        } else {
            Ok(Some(T::from_js_value(js_value)?))
        }
    }
}

impl<T: FromJsValue> FromJsValue for Box<T> {
    fn from_js_value(js_value: Value) -> Result<Self> {
        Ok(Box::new(T::from_js_value(js_value)?))
    }
}

impl<T: FromJsValue> FromJsValue for Vec<T> {
    fn from_js_value(js_value: Value) -> Result<Self> {
        let len = js_value.length()? as usize;
        let mut vec = Vec::with_capacity(len);
        for i in 0..len {
            vec.push(T::from_js_value(js_value.get_property(&i.to_string())?)?)
        }
        Ok(vec)
    }
}

fn iter_fields<K, V>(js_value: Value) -> Result<impl Iterator<Item = Result<(K, V)>>>
where
    K: FromJsValue,
    V: FromJsValue,
{
    let mut iter = js_value
        .entries()
        .or(Err(Error::Expect("map-like object")))?;
    Ok(core::iter::from_fn(move || -> Option<Result<(K, V)>> {
        let (key, value) = opt_try!(iter.next()?);
        let key = match K::from_js_value(key) {
            Ok(k) => k,
            Err(err) => return Some(Err(err)),
        };
        let value = match V::from_js_value(value) {
            Ok(v) => v,
            Err(err) => return Some(Err(err)),
        };
        Some(Ok((key, value)))
    }))
}

impl<K, V> FromJsValue for BTreeMap<K, V>
where
    K: FromJsValue + Ord,
    V: FromJsValue,
{
    fn from_js_value(js_value: Value) -> Result<Self> {
        iter_fields(js_value)?.collect()
    }
}

pub trait ToJsValue {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value>;
}

impl ToJsValue for Value {
    fn to_js_value(&self, _ctx: *mut c::JSContext) -> Result<Value> {
        Ok(self.clone())
    }
}

macro_rules! impl_to_js_for {
    ($t: ident, $encode_fn: ident) => {
        impl ToJsValue for $t {
            fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
                Ok(Value::$encode_fn(ctx, *self))
            }
        }
    };
}

impl_to_js_for!(i8, from_i8);
impl_to_js_for!(i16, from_i16);
impl_to_js_for!(i32, from_i32);
impl_to_js_for!(i64, from_i64);
impl_to_js_for!(u8, from_u8);
impl_to_js_for!(u16, from_u16);
impl_to_js_for!(u32, from_u32);
impl_to_js_for!(u64, from_u64);
impl_to_js_for!(f32, from_f32);
impl_to_js_for!(f64, from_f64);
impl_to_js_for!(i128, from_i128);
impl_to_js_for!(u128, from_u128);
impl_to_js_for!(bool, from_bool);

impl ToJsValue for &str {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        Ok(Value::from_str(ctx, self))
    }
}
impl ToJsValue for String {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        Ok(Value::from_str(ctx, self))
    }
}

impl ToJsValue for () {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        Ok(Value::null(ctx))
    }
}

impl<T: ToJsValue> ToJsValue for Option<T> {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        match self {
            Some(value) => value.to_js_value(ctx),
            None => Ok(Value::null(ctx)),
        }
    }
}

impl<T: ToJsValue> ToJsValue for Box<T> {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        self.as_ref().to_js_value(ctx)
    }
}

impl<T: ToJsValue> ToJsValue for Vec<T> {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        let js_array = Value::new_array(ctx);
        for value in self.iter() {
            js_array.array_push(&value.to_js_value(ctx)?)?;
        }
        Ok(js_array)
    }
}

impl<V: ToJsValue> ToJsValue for BTreeMap<String, V> {
    fn to_js_value(&self, ctx: *mut c::JSContext) -> Result<Value> {
        let js_object = Value::new_object(ctx);
        for (key, value) in self.iter() {
            js_object.set_property(&key, &value.to_js_value(ctx)?)?;
        }
        Ok(js_object)
    }
}
