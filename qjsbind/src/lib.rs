#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub use qjs_sys::c;
pub use error::{Error, Result};
pub use value::Value;
pub use traits::{FromJsValue, ToJsValue};

#[macro_use]
mod macros;
mod error;
mod traits;
mod value;
mod impls;

mod test {
    use qjsbind_derive::{FromJsValue, ToJsValue};

    #[derive(Debug, FromJsValue, ToJsValue)]
    pub struct HttpRequest {
        pub method: String,
        pub url: String,
        pub headers: Vec<(String, String)>,
        pub body: Vec<u8>,
    }
}
