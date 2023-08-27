use alloc::vec::Vec;
use qjs::{AsBytes, JsUint8Array};
use sha1::{Digest, Sha1};

#[qjs::host_call]
pub fn sha1(data: JsUint8Array) -> AsBytes<Vec<u8>> {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    AsBytes(hasher.finalize().to_vec())
}
