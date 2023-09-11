use alloc::string::{String, ToString};
use core::fmt::Display;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Static(&'static str),
    Expect(&'static str),
    ExpectLen(String, usize),
    JsException(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Custom(s) => f.write_str(s),
            Error::Static(s) => f.write_str(s),
            Error::Expect(s) => write!(f, "expect {}", s),
            Error::JsException(e) => f.write_str(e),
            Error::ExpectLen(s, l) => write!(f, "expect [{s};{l}]"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.to_string()
    }
}
