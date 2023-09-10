use alloc::{string::String, vec::Vec};
use base64::{engine::general_purpose, Engine as _};
use js::{AsBytes, JsString, JsUint8Array, Result};

#[js::host_call]
pub fn encode(data: JsUint8Array, pad: bool) -> String {
    if pad {
        general_purpose::STANDARD.encode(data.as_bytes())
    } else {
        general_purpose::STANDARD_NO_PAD.encode(data.as_bytes())
    }
}
#[js::host_call]
pub fn decode(base64_str: JsString, pad: bool) -> Result<AsBytes<Vec<u8>>> {
    if pad {
        general_purpose::STANDARD
            .decode(base64_str.as_str())
            .map(AsBytes)
            .or(Err(js::Error::Expect("padded base64 string")))
    } else {
        general_purpose::STANDARD_NO_PAD
            .decode(base64_str.as_str())
            .map(AsBytes)
            .or(Err(js::Error::Expect("base64 string")))
    }
}
