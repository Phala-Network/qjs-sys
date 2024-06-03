use alloc::{string::String, vec::Vec};
use js::{AsBytes, BytesOrString, JsString, Result, ErrorContext};

#[js::host_call]
pub fn encode(data: BytesOrString, add_prefix: Option<bool>) -> String {
    let prefix = if add_prefix.unwrap_or(false) {
        "0x"
    } else {
        ""
    };
    alloc::format!("{}{}", prefix, hex_fmt::HexFmt(data))
}
#[js::host_call]
pub fn decode(hex_str: JsString) -> Result<AsBytes<Vec<u8>>> {
    let hex_str = hex_str.as_str().trim_start_matches("0x");
    hex::decode(hex_str)
        .map(AsBytes)
        .context("invalid hex string")
}
