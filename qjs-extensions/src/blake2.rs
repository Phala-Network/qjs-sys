use alloc::vec::Vec;
use blake2::{
    digest::typenum::{U16, U32, U64},
    Blake2b, Blake2s, Digest,
};
use qjs::{AsBytes, JsUint8Array};

fn blake2b128_encode(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::<U16>::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
fn blake2b256_encode(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
fn blake2b512_encode(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::<U64>::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
fn blake2s256_encode(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2s::<U32>::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[qjs::host_call]
pub fn blake2b_128(data: JsUint8Array) -> AsBytes<Vec<u8>> {
    AsBytes(blake2b128_encode(data.as_bytes()))
}

#[qjs::host_call]
pub fn blake2b_256(data: JsUint8Array) -> AsBytes<Vec<u8>> {
    AsBytes(blake2b256_encode(data.as_bytes()))
}

#[qjs::host_call]
pub fn blake2b_512(data: JsUint8Array) -> AsBytes<Vec<u8>> {
    AsBytes(blake2b512_encode(data.as_bytes()))
}

#[qjs::host_call]
pub fn blake2s_256(data: JsUint8Array) -> AsBytes<Vec<u8>> {
    AsBytes(blake2s256_encode(data.as_bytes()))
}
