use alloc::string::String;
use core::fmt::Display;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Static(&'static str),
    Expect(&'static str),
    ExpectLen(&'static str, usize),
    JsException,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Custom(s) => f.write_str(s),
            Error::Static(s) => f.write_str(s),
            Error::Expect(s) => write!(f, "expect {}", s),
            Error::JsException => f.write_str("js exception"),
            Error::ExpectLen(s, l) => write!(f, "expect {} of length {}", s, l),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
