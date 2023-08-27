use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use qjs::{AsBytes, JsString, JsUint8Array, Result};

#[qjs::host_call]
pub fn encode(data: JsString) -> AsBytes<Vec<u8>> {
    AsBytes(data.as_str().as_bytes().to_vec())
}

#[qjs::host_call]
pub fn decode(utf8_data: JsUint8Array) -> Result<String> {
    let utf8_data = utf8_data.as_bytes();
    let utf8_str = core::str::from_utf8(utf8_data).or(Err(qjs::Error::Expect("utf8 string")))?;
    Ok(utf8_str.to_string())
}
