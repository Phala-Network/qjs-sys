#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub use as_bytes::{decode_as_bytes, encode_as_bytes};
pub use error::{Error, Result};
pub use eval::{eval};
pub use host_function::{call_host_function, Function, HostFunction};
pub use qjs_sys as sys;
pub use qjs_sys::c;
pub use qjsbind_derive::{FromJsValue, ToJsValue};
pub use traits::{FromJsValue, ToJsValue, ArgList};
pub use utils::{compile, ctx_get_exception_str, ctx_to_str, ctx_to_string, js_throw_type_error};
pub use value::Value;
pub use impls::AsBytes;

#[macro_use]
mod macros;
mod as_bytes;
mod error;
mod eval;
mod host_function;
mod impls;
mod traits;
mod utils;
mod value;

mod test {
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };
    use qjsbind_derive::{FromJsValue, ToJsValue};

    use crate::Value;

    #[derive(FromJsValue, ToJsValue)]
    #[qjsbind(rename_all = "camelCase")]
    pub struct HttpRequest {
        #[qjsbind(default = "default_method")]
        pub method: String,
        pub url: String,
        pub headers: Vec<(String, String)>,
        #[qjsbind(default)]
        pub body: Vec<u8>,
        pub foo_bar: Vec<u8>,
        pub opaque: Value,
    }

    fn default_method() -> String {
        "GET".to_string()
    }
}
