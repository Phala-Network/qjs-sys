use alloc::vec::Vec;

use crate::{self as js, FromJsValue, ToJsValue};

use super::{Error, Result, Value};

pub fn encode_as_bytes<T: AsRef<[u8]>>(ctx: &js::Context, data: &T) -> Result<Value> {
    Ok(Value::from_bytes(ctx, data.as_ref()))
}

pub fn decode_as_bytes<T>(js_value: Value) -> Result<T>
where
    Vec<u8>: TryInto<T>,
{
    let bytes = js_value.decode_bytes()?;
    bytes.try_into().or(Err(Error::Expect("try from bytes")))
}

pub fn decode_as_bytes_maybe_hex<T>(js_value: Value) -> Result<T>
where
    Vec<u8>: TryInto<T>,
{
    let bytes = js_value.decode_bytes_maybe_hex()?;
    bytes.try_into().or(Err(Error::Expect("try from bytes")))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsBytes<T>(pub T);

impl<T: AsRef<[u8]>> ToJsValue for AsBytes<T> {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        encode_as_bytes(ctx, &self.0)
    }
}

impl<T> FromJsValue for AsBytes<T>
where
    Vec<u8>: TryInto<T>,
{
    fn from_js_value(value: Value) -> Result<Self> {
        Ok(Self(decode_as_bytes(value)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytesOrHex<T>(pub T);

impl<T: AsRef<[u8]>> ToJsValue for BytesOrHex<T> {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        encode_as_bytes(ctx, &self.0)
    }
}

impl<T> FromJsValue for BytesOrHex<T>
where
    Vec<u8>: Into<T>,
{
    fn from_js_value(value: Value) -> Result<Self> {
        Ok(Self(decode_as_bytes_maybe_hex(value)?))
    }
}
