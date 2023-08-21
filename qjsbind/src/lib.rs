#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub use as_bytes::{decode_as_bytes, encode_as_bytes};
pub use error::{Error, Result};
pub use host_function::{call_host_function, Function, HostFunction};
pub use qjs_sys::c;
pub use qjsbind_derive::{FromJsValue, ToJsValue};
pub use traits::{FromJsValue, ToJsValue};
pub use value::Value;

#[macro_use]
mod macros;
mod as_bytes;
mod error;
mod host_function;
mod impls;
mod traits;
mod value;

mod test {
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
