use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use js::{AsBytes, ErrorContext, JsString, JsUint8Array, Result};

#[derive(Debug, js::ToJsValue, Default)]
pub struct EncodeProgress {
    read: usize,
    written: usize,
}

#[js::host_call]
pub fn encode(data: JsString) -> AsBytes<Vec<u8>> {
    AsBytes(data.as_str().as_bytes().to_vec())
}

#[js::host_call]
pub fn encode_into(data: JsString, buf: JsUint8Array) -> EncodeProgress {
    let utf8_bytes = data.as_str().as_bytes();
    if !buf.fill_with_bytes(utf8_bytes) {
        return Default::default();
    }
    EncodeProgress {
        read: data.as_str().len(),
        written: utf8_bytes.len(),
    }
}

#[js::host_call]
pub fn decode(utf8_data: js::Bytes) -> Result<String> {
    let utf8_data = utf8_data.as_bytes();
    let utf8_str = core::str::from_utf8(utf8_data).context("invalid utf-8 data")?;
    Ok(utf8_str.to_string())
}
