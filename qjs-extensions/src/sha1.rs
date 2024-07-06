use js::{AsBytes, BytesOrString};
use sha1::{Digest, Sha1};

#[js::host_call]
pub fn sha1(data: BytesOrString) -> AsBytes<[u8; 20]> {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    AsBytes(hasher.finalize().into())
}
