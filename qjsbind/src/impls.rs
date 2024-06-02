use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use anyhow::anyhow;
use tinyvec::TinyVec;

use super::{FromArgs, FromJsValue, Result, ToArgs, ToJsValue, Value};
use crate::{
    self as js,
    error::{expect_js_value, JsResultExt},
};

impl FromJsValue for Value {
    fn from_js_value(js_value: Value) -> Result<Self> {
        Ok(js_value)
    }
}

macro_rules! impl_from_for {
    ($t: ident, $decode_fn: ident) => {
        impl FromJsValue for $t {
            fn from_js_value(js_value: Value) -> Result<Self> {
                js_value
                    .$decode_fn()
                    .expect_js_value(&js_value, stringify!($t))
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
impl_from_for!(usize, decode_usize);

impl FromJsValue for () {
    fn from_js_value(js_value: Value) -> Result<Self> {
        if js_value.is_null_or_undefined() {
            Ok(())
        } else {
            Err(expect_js_value(&js_value, "()"))
        }
    }
}

macro_rules! impl_from_for_tuple {
    ($($t: ident),*) => {
        impl<$($t),*> FromJsValue for ($($t,)*)
        where
            $($t: FromJsValue),*
        {
            fn from_js_value(js_value: Value) -> Result<Self> {
                let mut iter = iter_values(js_value)?;
                Ok(($($t::from_js_value(iter.next().ok_or_else(|| anyhow!("expect tuple"))??)?,)*))
            }
        }
    };
}
impl_from_for_tuple!(A);
impl_from_for_tuple!(A, B);
impl_from_for_tuple!(A, B, C);
impl_from_for_tuple!(A, B, C, D);
impl_from_for_tuple!(A, B, C, D, E);
impl_from_for_tuple!(A, B, C, D, E, F);
impl_from_for_tuple!(A, B, C, D, E, F, G);
impl_from_for_tuple!(A, B, C, D, E, F, G, H);
impl_from_for_tuple!(A, B, C, D, E, F, G, H, I);

impl<T: FromJsValue> FromJsValue for Option<T> {
    fn from_js_value(js_value: Value) -> Result<Self> {
        if js_value.is_null_or_undefined() {
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
        iter_values(js_value)?.collect()
    }
}

fn iter_values<V: FromJsValue>(js_value: Value) -> Result<impl Iterator<Item = Result<V>>> {
    let mut iter = js_value
        .values()
        .expect_js_value(&js_value, "array-like object")?;
    Ok(core::iter::from_fn(move || -> Option<Result<V>> {
        let value = opt_try!(iter.next()?);
        Some(V::from_js_value(value))
    }))
}

fn iter_fields<K, V>(js_value: Value) -> Result<impl Iterator<Item = Result<(K, V)>>>
where
    K: FromJsValue,
    V: FromJsValue,
{
    let mut iter = js_value
        .entries()
        .expect_js_value(&js_value, "map-like object")?;
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

impl<const N: usize, T: FromJsValue + Default> FromJsValue for [T; N] {
    fn from_js_value(js_value: Value) -> Result<Self> {
        let mut iter = iter_values(js_value)?;
        let mut array: Vec<T> = vec![];
        for _ in 0..N {
            array.push(
                iter.next()
                    .ok_or_else(|| anyhow!("expect [{}: {}]", crate::type_name::<T>(), N))??,
            );
        }
        Ok(array.try_into().ok().expect("BUG: array length mismatch"))
    }
}

impl ToJsValue for Value {
    fn to_js_value(&self, _ctx: &js::Context) -> Result<Value> {
        Ok(self.clone())
    }
}

macro_rules! impl_to_js_for {
    ($t: ident, $encode_fn: ident) => {
        impl ToJsValue for $t {
            fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
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
impl_to_js_for!(usize, from_usize);

impl ToJsValue for &str {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        Ok(Value::from_str(ctx, self))
    }
}
impl ToJsValue for String {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        Ok(Value::from_str(ctx, self))
    }
}

impl ToJsValue for () {
    fn to_js_value(&self, _ctx: &js::Context) -> Result<Value> {
        Ok(Value::null())
    }
}

macro_rules! impl_to_js_for_tuple {
    ($($t: ident),*) => {
        impl<$($t: ToJsValue),*> ToJsValue for ($($t,)*) {
            fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
                let js_array = Value::new_array(ctx);
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                $(
                    js_array.array_push(&$t.to_js_value(ctx)?)?;
                )*
                Ok(js_array)
            }
        }
    };
}

impl_to_js_for_tuple!(A);
impl_to_js_for_tuple!(A, B);
impl_to_js_for_tuple!(A, B, C);
impl_to_js_for_tuple!(A, B, C, D);
impl_to_js_for_tuple!(A, B, C, D, E);
impl_to_js_for_tuple!(A, B, C, D, E, F);
impl_to_js_for_tuple!(A, B, C, D, E, F, G);
impl_to_js_for_tuple!(A, B, C, D, E, F, G, H);
impl_to_js_for_tuple!(A, B, C, D, E, F, G, H, I);

impl<T: ToJsValue> ToJsValue for Option<T> {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        match self {
            Some(value) => value.to_js_value(ctx),
            None => Ok(Value::null()),
        }
    }
}

impl<T: ToJsValue> ToJsValue for Box<T> {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        self.as_ref().to_js_value(ctx)
    }
}

impl<T: ToJsValue> ToJsValue for [T] {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        let js_array = Value::new_array(ctx);
        for value in self.iter() {
            js_array.array_push(&value.to_js_value(ctx)?)?;
        }
        Ok(js_array)
    }
}

impl<T: ToJsValue> ToJsValue for Vec<T> {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        self.as_slice().to_js_value(ctx)
    }
}

impl<const N: usize, T: ToJsValue> ToJsValue for [T; N] {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        self.as_slice().to_js_value(ctx)
    }
}

impl<V: ToJsValue> ToJsValue for BTreeMap<String, V> {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        let js_object = Value::new_object(ctx, "BTMObject");
        for (key, value) in self.iter() {
            js_object.set_property(key, &value.to_js_value(ctx)?)?;
        }
        Ok(js_object)
    }
}

macro_rules! impl_arglist_for {
    (($($t: ident),*)) => {
        impl<$($t: FromJsValue),*> FromArgs for ($($t,)*) {
            fn from_args(argv: &[Value]) -> Result<Self> {
                #[allow(unused_mut)]
                #[allow(unused_variables)]
                let mut iter = argv.iter();
                Ok(($(
                    $t::from_js_value(iter.next().cloned().unwrap_or_default())?,
                )*))
            }
        }
        impl<$($t: ToJsValue),*> ToArgs for ($($t,)*) {
            fn to_args(&self, ctx: &js::Context) -> Result<TinyVec<[Value; 8]>> {
                #[allow(unused_mut)]
                let mut args = TinyVec::new();
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                #[allow(unused_variables)]
                let ctx = ctx;
                $(
                    args.push($t.to_js_value(ctx)?);
                )*
                Ok(args)
            }
        }
    };
}

impl_arglist_for!(());
impl_arglist_for!((A));
impl_arglist_for!((A, B));
impl_arglist_for!((A, B, C));
impl_arglist_for!((A, B, C, D));
impl_arglist_for!((A, B, C, D, E));
impl_arglist_for!((A, B, C, D, E, F));
impl_arglist_for!((A, B, C, D, E, F, G));
impl_arglist_for!((A, B, C, D, E, F, G, H));
