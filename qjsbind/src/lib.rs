#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub extern crate alloc;

pub use as_bytes::{
    decode_as_bytes, decode_as_bytes_maybe_hex, encode_as_bytes, AsBytes, Bytes, BytesOrHex,
    BytesOrString,
};
pub use engine::{Context, Runtime};
pub use error::{
    no_std_context::NoStdContext, AnyError, Context as ErrorContext, Error, JsResultExt, Result,
};
pub use eval::{eval, Code};
pub use host_function::convert_host_call_result;
pub use js_string::JsString;
pub use js_u8array::JsUint8Array;
pub use js_arraybuffer::JsArrayBuffer;
pub use native_object::{
    GcMark, IntoNativeObject, Named, Native, NativeClass, NativeValueRef, NativeValueRefMut, NoGc,
};
pub use qjs_sys as sys;
pub use qjs_sys::c;
pub use qjsbind_derive::{host_call, qjsbind, FromJsValue, GcMark, ToJsValue};
pub use traits::{FromArgs, FromJsContext, FromJsValue, OwnedRawArgs, ToArgs, ToJsValue};
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
mod js_arraybuffer;
mod native_object;
mod opaque_value;
mod traits;
mod utils;
mod value;

#[cfg(feature = "json")]
mod json_value;

#[cfg(feature = "tynm")]
use tynm::type_name;

#[cfg(not(feature = "tynm"))]
fn type_name<T>() -> alloc::string::String {
    core::any::type_name::<T>().into()
}
