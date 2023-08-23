use core::ptr::NonNull;

use alloc::vec::Vec;

use super::{c, Result, Value};

pub fn encode_as_bytes<T: AsRef<[u8]>>(ctx: NonNull<c::JSContext>, data: &T) -> Result<Value> {
    Ok(Value::from_bytes(ctx, data.as_ref()))
}

pub fn decode_as_bytes<T>(js_value: Value) -> Result<T>
where
    Vec<u8>: Into<T>,
{
    let bytes = js_value.decode_bytes()?;
    Ok(bytes.into())
}
