use alloc::vec::Vec;
use js::{AsBytes, BytesOrString};
pub use sha3::{Digest, Sha3_256, Sha3_512};

#[js::host_call]
pub fn sha3_256(data: BytesOrString) -> AsBytes<Vec<u8>> {
    let mut hasher = Sha3_256::new();
    hasher.update(data.as_bytes());
    AsBytes(hasher.finalize().to_vec())
}

#[js::host_call]
pub fn sha3_512(data: BytesOrString) -> AsBytes<Vec<u8>> {
    let mut hasher = Sha3_512::new();
    hasher.update(data.as_bytes());
    AsBytes(hasher.finalize().to_vec())
}
