use alloc::{string::String, vec::Vec};
use base64::{engine::general_purpose, Engine as _};
use js::{AsBytes, BytesOrString, ErrorContext, JsString, Result};

#[js::host_call]
pub fn encode(data: BytesOrString, pad: bool) -> String {
    b64_encode(data, pad)
}

pub fn b64_encode<T: AsRef<[u8]>>(data: T, pad: bool) -> String {
    if pad {
        general_purpose::STANDARD.encode(data)
    } else {
        general_purpose::STANDARD_NO_PAD.encode(data)
    }
}

#[js::host_call]
pub fn decode(base64_str: JsString, pad: bool) -> Result<AsBytes<Vec<u8>>> {
    b64_decode(base64_str.as_str(), pad).map(AsBytes)
}

pub fn b64_decode<T: AsRef<[u8]>>(encoded: T, pad: bool) -> Result<Vec<u8>> {
    if pad {
        general_purpose::STANDARD
            .decode(encoded)
            .context("invalid base64 string")
    } else {
        general_purpose::STANDARD_NO_PAD
            .decode(encoded)
            .context("invalid base64 string")
    }
}
