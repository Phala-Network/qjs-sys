use core::fmt::{Debug, Display};

pub use anyhow::{Error, Result};

#[cfg(feature = "std")]
pub use anyhow::Context;
#[cfg(not(feature = "std"))]
pub use no_std_context::NoStdContext as Context;

pub mod no_std_context {
    use super::*;

    pub trait NoStdContext<T, E> {
        fn context<C>(self, context: C) -> Result<T, Error>
        where
            C: Display + Send + Sync + 'static;
        fn with_context<C, F>(self, context: F) -> Result<T, Error>
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C;
    }

    impl<T, E> NoStdContext<T, E> for Result<T, E>
    where
        E: AnyError + Send + Sync + 'static,
    {
        fn context<C>(self, context: C) -> Result<T, Error>
        where
            C: Display + Send + Sync + 'static,
        {
            anyhow::Context::context(self.map_err(Error::msg), context)
        }

        fn with_context<C, F>(self, context: F) -> Result<T, Error>
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            anyhow::Context::with_context(self.map_err(Error::msg), context)
        }
    }

    impl<T> NoStdContext<T, Error> for Option<T> {
        fn context<C>(self, context: C) -> Result<T, Error>
        where
            C: Display + Send + Sync + 'static,
        {
            anyhow::Context::context(self, context)
        }

        fn with_context<C, F>(self, context: F) -> Result<T, Error>
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            anyhow::Context::with_context(self, context)
        }
    }
}

pub trait AnyError: Debug + Display + Send + Sync + 'static {}
impl<T> AnyError for T where T: Debug + Display + Send + Sync + 'static {}

pub trait JsResultExt {
    type T;
    fn expect_js_value(self, value: &crate::Value, tobe: &str) -> Result<Self::T>;
}

impl<T, E> JsResultExt for Result<T, E>
where
    E: AnyError,
{
    type T = T;
    fn expect_js_value(self, value: &crate::Value, tobe: &str) -> Result<T> {
        self.map_err(Error::msg)
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
