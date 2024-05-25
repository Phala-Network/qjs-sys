#![cfg_attr(not(any(feature = "std", test)), no_std)]
extern crate alloc;

#[cfg(feature = "base64")]
pub mod base64;
#[cfg(feature = "blake2")]
pub mod blake2;
#[cfg(feature = "hex")]
pub mod hex;
#[cfg(feature = "sha1")]
pub mod sha1;
#[cfg(feature = "sha2")]
pub mod sha2;
#[cfg(feature = "sha3")]
pub mod sha3;
pub mod utf8;

#[cfg(feature = "scale")]
pub mod scale;
#[cfg(feature = "scale2")]
pub mod scale2;

#[cfg(feature = "crypto")]
pub mod crypto;

pub mod repr;
