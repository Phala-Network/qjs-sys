#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub use as_bytes::{decode_as_bytes, encode_as_bytes};
pub use error::{Error, Result};
pub use eval::{eval, JsCode};
pub use host_function::{call_host_function, host_fn_stub, Function, HostFunction};
pub use impls::AsBytes;
pub use qjs_sys as sys;
pub use qjs_sys::c;
pub use qjsbind_derive::{host_call, FromJsValue, ToJsValue};
pub use traits::{FromArgs, FromJsValue, OwnedRawArgs, ToArgs, ToJsValue};
pub use utils::{compile, ctx_get_exception_str, ctx_to_str, ctx_to_string, js_throw_type_error};
pub use value::{get_global, Value};

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
