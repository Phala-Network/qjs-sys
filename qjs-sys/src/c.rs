#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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
