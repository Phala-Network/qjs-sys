
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    pub type JSValueConst = JSValue;
    pub const JS_NULL: JSValue = macros::JS_MKVAL(JS_TAG_NULL, 0);
    pub const JS_UNDEFINED: JSValue = macros::JS_MKVAL(JS_TAG_UNDEFINED, 0);
    pub const JS_FALSE: JSValue = macros::JS_MKVAL(JS_TAG_BOOL, 0);
    pub const JS_TRUE: JSValue = macros::JS_MKVAL(JS_TAG_BOOL, 1);
    pub const JS_EXCEPTION: JSValue = macros::JS_MKVAL(JS_TAG_EXCEPTION, 0);
    pub const JS_UNINITIALIZED: JSValue = macros::JS_MKVAL(JS_TAG_UNINITIALIZED, 0);

    #[cfg(target_pointer_width = "64")]
    mod macros {
        use super::*;

        pub const fn JS_MKVAL(tag: i32, val: i32) -> JSValue {
            JSValue {
                u: JSValueUnion { int32: val },
                tag: tag as _,
            }
        }
    }

    #[cfg(target_pointer_width = "32")]
    mod macros {
        pub const fn JS_MKVAL(tag: i32, val: i32) -> super::JSValue {
            ((tag as u32 as u64) << 32) as u64 | (val as u32 as u64)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub use super::inline_fns::*;