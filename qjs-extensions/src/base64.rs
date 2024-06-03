use alloc::{string::String, vec::Vec};
use base64::{engine::general_purpose, Engine as _};
use js::{AsBytes, BytesOrString, ErrorContext, JsString, Result};

#[js::host_call]
pub fn encode(data: BytesOrString, pad: bool) -> String {
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
            .context("invalid base64 string")
    } else {
        general_purpose::STANDARD_NO_PAD
            .decode(base64_str.as_str())
            .map(AsBytes)
            .context("invalid base64 string")
    }
}
