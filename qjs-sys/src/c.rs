#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::needless_return)]
#![allow(clippy::let_and_return)]

pub type JSValueConst = JSValue;
pub use JS_MakeValue as JS_MKVAL;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use super::inline_fns::*;

pub fn is_exception(v: JSValue) -> bool {
    unsafe { JS_IsException(v) != 0 }
}

pub fn is_undefined(v: JSValue) -> bool {
    unsafe { JS_IsUndefined(v) != 0 }
}
