use alloc::vec::Vec;

use crate::{self as js, FromJsValue, JsString, JsUint8Array, ToJsValue};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsBytes<T>(pub T);
impl<T> From<T> for AsBytes<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BytesOrHex<T>(pub T);
impl<T> From<T> for BytesOrHex<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

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

#[derive(Debug)]
pub enum BytesOrString {
    Uint8Array(JsUint8Array),
    String(JsString),
    Bytes(Vec<u8>),
}

impl Default for BytesOrString {
    fn default() -> Self {
        Self::Bytes(Vec::new())
    }
}

impl AsRef<[u8]> for BytesOrString {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl BytesOrString {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Uint8Array(bytes) => bytes.as_bytes(),
            Self::String(hex) => hex.as_str().as_bytes(),
            Self::Bytes(bytes) => bytes.as_slice(),
        }
    }
}

impl FromJsValue for BytesOrString {
    fn from_js_value(value: Value) -> Result<Self> {
        if value.is_string() {
            return Ok(Self::String(FromJsValue::from_js_value(value)?));
        }
        if value.is_uint8_array() {
            return Ok(Self::Uint8Array(FromJsValue::from_js_value(value)?));
        }
        AsBytes::<Vec<u8>>::from_js_value(value).map(|v| Self::Bytes(v.0))
    }
}

impl ToJsValue for BytesOrString {
    fn to_js_value(&self, ctx: &js::Context) -> Result<Value> {
        match self {
            Self::Uint8Array(bytes) => Ok(bytes.to_js_value(ctx)?),
            Self::String(hex) => Ok(hex.to_js_value(ctx)?),
            Self::Bytes(bytes) => encode_as_bytes(ctx, bytes),
        }
    }
}
