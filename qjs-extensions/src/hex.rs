use alloc::{string::String, vec::Vec};
use qjs::{AsBytes, JsString, JsUint8Array, Result};

#[qjs::host_call]
pub fn encode(data: JsUint8Array, add_prefix: Option<bool>) -> String {
    let prefix = if add_prefix.unwrap_or(false) {
        "0x"
    } else {
        ""
    };
    alloc::format!("{}{}", prefix, hex_fmt::HexFmt(data.as_bytes()))
}
#[qjs::host_call]
pub fn decode(hex_str: JsString) -> Result<AsBytes<Vec<u8>>> {
    let hex_str = hex_str.as_str().trim_start_matches("0x");
    hex::decode(hex_str)
        .map(AsBytes)
        .or(Err(qjs::Error::Expect("hex string")))
}
