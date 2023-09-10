use alloc::{string::String, vec::Vec};
use js::{AsBytes, JsString, JsUint8Array, Result};

#[js::host_call]
pub fn encode(data: JsUint8Array, add_prefix: Option<bool>) -> String {
    let prefix = if add_prefix.unwrap_or(false) {
        "0x"
    } else {
        ""
    };
    alloc::format!("{}{}", prefix, hex_fmt::HexFmt(data.as_bytes()))
}
#[js::host_call]
pub fn decode(hex_str: JsString) -> Result<AsBytes<Vec<u8>>> {
    let hex_str = hex_str.as_str().trim_start_matches("0x");
    hex::decode(hex_str)
        .map(AsBytes)
        .or(Err(js::Error::Expect("hex string")))
}
