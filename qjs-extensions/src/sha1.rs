use alloc::vec::Vec;
use js::{AsBytes, BytesOrString};
use sha1::{Digest, Sha1};

#[js::host_call]
pub fn sha1(data: BytesOrString) -> AsBytes<Vec<u8>> {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    AsBytes(hasher.finalize().to_vec())
}
