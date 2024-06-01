pub use anyhow::{Context, Error, Result};

pub trait JsResultExt {
    type T;
    fn expect_js_value(self, value: &crate::Value, tobe: &str) -> Result<Self::T>;
}

impl<T, E> JsResultExt for Result<T, E>
where
    Error: From<E>,
{
    type T = T;
    fn expect_js_value(self, value: &crate::Value, tobe: &str) -> Result<T> {
        self.map_err(Error::from)
            .context(format!("expect {tobe}, got {}", value.get_name()))
    }
}

impl<T> JsResultExt for Option<T> {
    type T = T;
    fn expect_js_value(self, value: &crate::Value, tobe: &str) -> Result<Self::T> {
        self.context(format!("expect {tobe}, got {}", value.get_name()))
    }
}

pub fn expect_js_value(value: &crate::Value, tobe: &str) -> Error {
    Error::msg(format!("expect {tobe}, got {}", value.get_name()))
}
