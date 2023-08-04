#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub type JSValueConst = JSValue;
pub const JS_NAN: JSValue = JS_MakeNAN();
pub const JS_NULL: JSValue = JS_MakeNULL();
pub const JS_UNDEFINED: JSValue = JS_MakeUNDEFINED();
pub const JS_FALSE: JSValue = JS_MakeFALSE();
pub const JS_TRUE: JSValue = JS_MakeTRUE();
pub const JS_EXCEPTION: JSValue = JS_MakeEXCEPTION();
pub const JS_UNINITIALIZED: JSValue = JS_MakeUNDEFINED();
pub use JS_MakeValue as JS_MKVAL;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use super::inline_fns::*;

pub fn is_exception(v: JSValue) -> bool {
    unsafe { JS_IsException(v) != 0 }
}
