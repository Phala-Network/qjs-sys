use alloc::vec::Vec;
use js::{AsBytes, BytesOrString};
use sha2::{Digest, Sha256};

#[js::host_call]
pub fn sha256(data: BytesOrString) -> AsBytes<Vec<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    AsBytes(hasher.finalize().to_vec())
}
