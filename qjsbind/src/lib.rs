#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub extern crate alloc;

pub use as_bytes::{
    decode_as_bytes, decode_as_bytes_maybe_hex, encode_as_bytes, AsBytes, BytesOrHex,
};
pub use engine::{Context, Runtime};
pub use error::{Error, Result};
pub use eval::{eval, Code};
pub use host_function::{call_host_function, Function, HostFunction};
pub use js_string::JsString;
pub use js_u8array::JsUint8Array;
pub use qjs_sys as sys;
pub use qjs_sys::c;
pub use qjsbind_derive::{host_call, FromJsValue, ToJsValue};
pub use traits::{FromArgs, FromJsValue, OwnedRawArgs, ToArgs, ToJsValue};
pub use utils::{compile, ctx_to_str, ctx_to_string, recursive_to_string};
pub use value::{get_global, Value};

#[macro_use]
mod macros;
mod as_bytes;
mod engine;
mod error;
mod eval;
mod host_function;
mod impls;
mod js_string;
mod js_u8array;
mod opaque_value;
mod traits;
mod utils;
mod value;
