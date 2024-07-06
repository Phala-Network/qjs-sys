use blake2::{
    digest::typenum::{U16, U32, U64},
    Blake2b, Blake2s, Digest,
};
use js::{AsBytes, BytesOrString};

fn blake2b128_encode(data: &[u8]) -> [u8; 16] {
    let mut hasher = Blake2b::<U16>::new();
    hasher.update(data);
    hasher.finalize().into()
}
fn blake2b256_encode(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(data);
    hasher.finalize().into()
}
fn blake2b512_encode(data: &[u8]) -> [u8; 64] {
    let mut hasher = Blake2b::<U64>::new();
    hasher.update(data);
    hasher.finalize().into()
}
fn blake2s256_encode(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake2s::<U32>::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[js::host_call]
pub fn blake2b_128(data: BytesOrString) -> AsBytes<[u8; 16]> {
    AsBytes(blake2b128_encode(data.as_bytes()))
}

#[js::host_call]
pub fn blake2b_256(data: BytesOrString) -> AsBytes<[u8; 32]> {
    AsBytes(blake2b256_encode(data.as_bytes()))
}

#[js::host_call]
pub fn blake2b_512(data: BytesOrString) -> AsBytes<[u8; 64]> {
    AsBytes(blake2b512_encode(data.as_bytes()))
}

#[js::host_call]
pub fn blake2s_256(data: BytesOrString) -> AsBytes<[u8; 32]> {
    AsBytes(blake2s256_encode(data.as_bytes()))
}
