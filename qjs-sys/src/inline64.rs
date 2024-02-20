#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![allow(unused_variables)]
use crate::c::*;
use crate::libc;
extern "C" {
    fn __JS_FreeValue(ctx: *mut JSContext, v: JSValue);
    fn __JS_FreeValueRT(rt: *mut JSRuntime, v: JSValue);
    fn JS_ToInt32(ctx: *mut JSContext, pres: *mut int32_t, val: JSValue) -> libc::c_int;
    fn JS_ToCStringLen2(
        ctx: *mut JSContext,
        plen: *mut size_t,
        val1: JSValue,
        cesu8: libc::c_int,
    ) -> *const libc::c_char;
    fn JS_GetPropertyInternal(
        ctx: *mut JSContext,
        obj: JSValue,
        prop: JSAtom,
        receiver: JSValue,
        throw_ref_error: libc::c_int,
    ) -> JSValue;
    fn JS_SetPropertyInternal(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
        val: JSValue,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn JS_NewCFunction2(
        ctx: *mut JSContext,
        func: Option<JSCFunction>,
        name: *const libc::c_char,
        length: libc::c_int,
        cproto: JSCFunctionEnum,
        magic: libc::c_int,
    ) -> JSValue;
    fn JS_NewCFunction2Len(
        ctx: *mut JSContext,
        func: Option<JSCFunction>,
        name: *const libc::c_char,
        name_length: libc::c_int,
        length: libc::c_int,
        cproto: JSCFunctionEnum,
        magic: libc::c_int,
    ) -> JSValue;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type size_t = usize;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type JSAtom = uint32_t;
pub type C2RustUnnamed = libc::c_int;
pub const JS_TAG_FLOAT64: C2RustUnnamed = 7;
pub const JS_TAG_EXCEPTION: C2RustUnnamed = 6;
pub const JS_TAG_CATCH_OFFSET: C2RustUnnamed = 5;
pub const JS_TAG_UNINITIALIZED: C2RustUnnamed = 4;
pub const JS_TAG_UNDEFINED: C2RustUnnamed = 3;
pub const JS_TAG_NULL: C2RustUnnamed = 2;
pub const JS_TAG_BOOL: C2RustUnnamed = 1;
pub const JS_TAG_INT: C2RustUnnamed = 0;
pub const JS_TAG_OBJECT: C2RustUnnamed = -1;
pub const JS_TAG_FUNCTION_BYTECODE: C2RustUnnamed = -2;
pub const JS_TAG_MODULE: C2RustUnnamed = -3;
pub const JS_TAG_STRING: C2RustUnnamed = -7;
pub const JS_TAG_SYMBOL: C2RustUnnamed = -8;
pub const JS_TAG_BIG_INT: C2RustUnnamed = -9;
pub const JS_TAG_FIRST: C2RustUnnamed = -9;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSRefCountHeader {
    pub ref_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union JSValueUnion_ {
    pub int32: int32_t,
    pub float64: libc::c_double,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSValue_ {
    pub u: JSValueUnion,
    pub tag: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub d: libc::c_double,
    pub u64_0: uint64_t,
}
pub type JSCFunction =
    unsafe extern "C" fn(*mut JSContext, JSValue, libc::c_int, *mut JSValue) -> JSValue;
pub type JSCFunctionMagic = unsafe extern "C" fn(
    *mut JSContext,
    JSValue,
    libc::c_int,
    *mut JSValue,
    libc::c_int,
) -> JSValue;
pub type JSCFunctionEnum = libc::c_uint;
pub const JS_CFUNC_iterator_next: JSCFunctionEnum = 12;
pub const JS_CFUNC_setter_magic: JSCFunctionEnum = 11;
pub const JS_CFUNC_getter_magic: JSCFunctionEnum = 10;
pub const JS_CFUNC_setter: JSCFunctionEnum = 9;
pub const JS_CFUNC_getter: JSCFunctionEnum = 8;
pub const JS_CFUNC_f_f_f: JSCFunctionEnum = 7;
pub const JS_CFUNC_f_f: JSCFunctionEnum = 6;
pub const JS_CFUNC_constructor_or_func_magic: JSCFunctionEnum = 5;
pub const JS_CFUNC_constructor_or_func: JSCFunctionEnum = 4;
pub const JS_CFUNC_constructor_magic: JSCFunctionEnum = 3;
pub const JS_CFUNC_constructor: JSCFunctionEnum = 2;
pub const JS_CFUNC_generic_magic: JSCFunctionEnum = 1;
pub const JS_CFUNC_generic: JSCFunctionEnum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union JSCFunctionType {
    pub generic: Option<JSCFunction>,
    pub generic_magic: Option<
        unsafe extern "C" fn(
            *mut JSContext,
            JSValue,
            libc::c_int,
            *mut JSValue,
            libc::c_int,
        ) -> JSValue,
    >,
    pub constructor: Option<JSCFunction>,
    pub constructor_magic: Option<
        unsafe extern "C" fn(
            *mut JSContext,
            JSValue,
            libc::c_int,
            *mut JSValue,
            libc::c_int,
        ) -> JSValue,
    >,
    pub constructor_or_func: Option<JSCFunction>,
    pub f_f: Option<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    pub f_f_f: Option<unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double>,
    pub getter: Option<unsafe extern "C" fn(*mut JSContext, JSValue) -> JSValue>,
    pub setter: Option<unsafe extern "C" fn(*mut JSContext, JSValue, JSValue) -> JSValue>,
    pub getter_magic: Option<unsafe extern "C" fn(*mut JSContext, JSValue, libc::c_int) -> JSValue>,
    pub setter_magic:
        Option<unsafe extern "C" fn(*mut JSContext, JSValue, JSValue, libc::c_int) -> JSValue>,
    pub iterator_next: Option<
        unsafe extern "C" fn(
            *mut JSContext,
            JSValue,
            libc::c_int,
            *mut JSValue,
            *mut libc::c_int,
            libc::c_int,
        ) -> JSValue,
    >,
}
pub const NAN: libc::c_float = ::core::f32::NAN;
pub const JS_FLOAT64_NAN: libc::c_float = ::core::f32::NAN;
pub const JS_NAN: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            float64: JS_FLOAT64_NAN as libc::c_double,
        },
        tag: JS_TAG_FLOAT64 as libc::c_int as int64_t,
    };
    init
};
#[inline]
pub unsafe extern "C" fn JS_MakeFloat64(mut d: libc::c_double) -> JSValue {
    let mut v: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    v.tag = JS_TAG_FLOAT64 as libc::c_int as int64_t;
    v.u.float64 = d;
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_VALUE_IS_NAN(mut v: JSValue) -> libc::c_int {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { d: 0. };
    let mut current_block_0: u64;
    if v.tag != JS_TAG_FLOAT64 as libc::c_int as libc::c_long {
        current_block_0 = 11174649648027449784;
    } else {
        current_block_0 = 12237857397564741460;
    }
    match current_block_0 {
        12237857397564741460 => {}
        _ => return 0 as libc::c_int,
    }
    u.d = v.u.float64;
    return (u.u64_0 & 0x7fffffffffffffff as libc::c_long as libc::c_ulong
        > 0x7ff0000000000000 as libc::c_long as libc::c_ulong) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn __JS_NewFloat64(mut d: libc::c_double) -> JSValue {
    return JS_MakeFloat64(d);
}
pub const JS_NULL: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            int32: 0 as libc::c_int,
        },
        tag: JS_TAG_NULL as libc::c_int as int64_t,
    };
    init
};
pub const JS_UNDEFINED: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            int32: 0 as libc::c_int,
        },
        tag: JS_TAG_UNDEFINED as libc::c_int as int64_t,
    };
    init
};
pub const JS_FALSE: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            int32: 0 as libc::c_int,
        },
        tag: JS_TAG_BOOL as libc::c_int as int64_t,
    };
    init
};
pub const JS_TRUE: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            int32: 1 as libc::c_int,
        },
        tag: JS_TAG_BOOL as libc::c_int as int64_t,
    };
    init
};
pub const JS_EXCEPTION: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            int32: 0 as libc::c_int,
        },
        tag: JS_TAG_EXCEPTION as libc::c_int as int64_t,
    };
    init
};
pub const JS_UNINITIALIZED: JSValue = {
    let mut init = JSValue {
        u: JSValueUnion {
            int32: 0 as libc::c_int,
        },
        tag: JS_TAG_UNINITIALIZED as libc::c_int as int64_t,
    };
    init
};
pub const JS_PROP_CONFIGURABLE: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const JS_PROP_WRITABLE: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
pub const JS_PROP_ENUMERABLE: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
pub const JS_PROP_LENGTH: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const JS_PROP_TMASK: libc::c_int = (3 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_NORMAL: libc::c_int = (0 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_GETSET: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_VARREF: libc::c_int = (2 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_AUTOINIT: libc::c_int = (3 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_THROW: libc::c_int = (1 as libc::c_int) << 14 as libc::c_int;
#[inline(always)]
pub unsafe extern "C" fn JS_NewBool(mut ctx: *mut JSContext, mut val: libc::c_int) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion {
                int32: (val != 0 as libc::c_int) as libc::c_int,
            },
            tag: JS_TAG_BOOL as libc::c_int as int64_t,
        };
        init
    };
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewInt32(mut ctx: *mut JSContext, mut val: int32_t) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: val },
            tag: JS_TAG_INT as libc::c_int as int64_t,
        };
        init
    };
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewCatchOffset(mut ctx: *mut JSContext, mut val: int32_t) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: val },
            tag: JS_TAG_CATCH_OFFSET as libc::c_int as int64_t,
        };
        init
    };
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewInt64(mut ctx: *mut JSContext, mut val: int64_t) -> JSValue {
    let mut v: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut current_block_3: u64;
    if val == val as int32_t as libc::c_long {
        current_block_3 = 11174649648027449784;
    } else {
        current_block_3 = 10771263883588581193;
    }
    match current_block_3 {
        10771263883588581193 => {
            v = __JS_NewFloat64(val as libc::c_double);
        }
        _ => {
            v = JS_NewInt32(ctx, val as int32_t);
        }
    }
    return v;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewUint32(mut ctx: *mut JSContext, mut val: uint32_t) -> JSValue {
    let mut v: JSValue = JSValue {
        u: JSValueUnion { int32: 0 },
        tag: 0,
    };
    let mut current_block_3: u64;
    if val <= 0x7fffffff as libc::c_int as libc::c_uint {
        current_block_3 = 11174649648027449784;
    } else {
        current_block_3 = 10771263883588581193;
    }
    match current_block_3 {
        10771263883588581193 => {
            v = __JS_NewFloat64(val as libc::c_double);
        }
        _ => {
            v = JS_NewInt32(ctx, val as int32_t);
        }
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_IsNumber(mut v: JSValue) -> libc::c_int {
    let mut tag: libc::c_int = v.tag as int32_t;
    return (tag == JS_TAG_INT as libc::c_int
        || tag as libc::c_uint == JS_TAG_FLOAT64 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsBigInt(mut v: JSValue) -> libc::c_int {
    let mut tag: libc::c_int = v.tag as int32_t;
    return (tag == JS_TAG_BIG_INT as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsBool(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_BOOL as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsNull(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_NULL as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsUndefined(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_UNDEFINED as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsException(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_EXCEPTION as libc::c_int) as libc::c_int as libc::c_long
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsUninitialized(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_UNINITIALIZED as libc::c_int) as libc::c_int as libc::c_long
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsString(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_STRING as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsSymbol(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_SYMBOL as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsObject(mut v: JSValue) -> libc::c_int {
    return (v.tag as int32_t == JS_TAG_OBJECT as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_FreeValue(mut ctx: *mut JSContext, mut v: JSValue) {
    let mut current_block_3: u64;
    if v.tag as int32_t as libc::c_uint >= JS_TAG_FIRST as libc::c_int as libc::c_uint {
        current_block_3 = 9188997750422900590;
    } else {
        current_block_3 = 7502529970979898288;
    }
    match current_block_3 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
            let mut current_block_1: u64;
            let ref mut fresh0 = (*p).ref_count;
            *fresh0 -= 1;
            if *fresh0 <= 0 as libc::c_int {
                current_block_1 = 6239978542346980191;
            } else {
                current_block_1 = 10680521327981672866;
            }
            match current_block_1 {
                6239978542346980191 => {
                    __JS_FreeValue(ctx, v);
                }
                _ => {}
            }
        }
        _ => {}
    };
}
#[inline]
pub unsafe extern "C" fn JS_FreeValueRT(mut rt: *mut JSRuntime, mut v: JSValue) {
    let mut current_block_3: u64;
    if v.tag as int32_t as libc::c_uint >= JS_TAG_FIRST as libc::c_int as libc::c_uint {
        current_block_3 = 9188997750422900590;
    } else {
        current_block_3 = 7502529970979898288;
    }
    match current_block_3 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
            let mut current_block_1: u64;
            let ref mut fresh1 = (*p).ref_count;
            *fresh1 -= 1;
            if *fresh1 <= 0 as libc::c_int {
                current_block_1 = 6239978542346980191;
            } else {
                current_block_1 = 10680521327981672866;
            }
            match current_block_1 {
                6239978542346980191 => {
                    __JS_FreeValueRT(rt, v);
                }
                _ => {}
            }
        }
        _ => {}
    };
}
#[inline]
pub unsafe extern "C" fn JS_DupValue(mut ctx: *mut JSContext, mut v: JSValue) -> JSValue {
    let mut current_block_1: u64;
    if v.tag as int32_t as libc::c_uint >= JS_TAG_FIRST as libc::c_int as libc::c_uint {
        current_block_1 = 9188997750422900590;
    } else {
        current_block_1 = 14155750587950065367;
    }
    match current_block_1 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
            let ref mut fresh2 = (*p).ref_count;
            *fresh2 += 1;
        }
        _ => {}
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_DupValueRT(mut rt: *mut JSRuntime, mut v: JSValue) -> JSValue {
    let mut current_block_1: u64;
    if v.tag as int32_t as libc::c_uint >= JS_TAG_FIRST as libc::c_int as libc::c_uint {
        current_block_1 = 9188997750422900590;
    } else {
        current_block_1 = 14155750587950065367;
    }
    match current_block_1 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader = v.u.ptr as *mut JSRefCountHeader;
            let ref mut fresh3 = (*p).ref_count;
            *fresh3 += 1;
        }
        _ => {}
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_ToUint32(
    mut ctx: *mut JSContext,
    mut pres: *mut uint32_t,
    mut val: JSValue,
) -> libc::c_int {
    return JS_ToInt32(ctx, pres as *mut int32_t, val);
}
#[inline]
pub unsafe extern "C" fn JS_ToCStringLen(
    mut ctx: *mut JSContext,
    mut plen: *mut size_t,
    mut val1: JSValue,
) -> *const libc::c_char {
    return JS_ToCStringLen2(ctx, plen, val1, 0 as libc::c_int);
}
#[inline]
pub unsafe extern "C" fn JS_ToCString(
    mut ctx: *mut JSContext,
    mut val1: JSValue,
) -> *const libc::c_char {
    return JS_ToCStringLen2(ctx, NULL as *mut size_t, val1, 0 as libc::c_int);
}
#[inline(always)]
pub unsafe extern "C" fn JS_GetProperty(
    mut ctx: *mut JSContext,
    mut this_obj: JSValue,
    mut prop: JSAtom,
) -> JSValue {
    return JS_GetPropertyInternal(ctx, this_obj, prop, this_obj, 0 as libc::c_int);
}
#[inline]
pub unsafe extern "C" fn JS_SetProperty(
    mut ctx: *mut JSContext,
    mut this_obj: JSValue,
    mut prop: JSAtom,
    mut val: JSValue,
) -> libc::c_int {
    return JS_SetPropertyInternal(ctx, this_obj, prop, val, JS_PROP_THROW);
}
#[inline]
pub unsafe extern "C" fn JS_NewCFunction(
    mut ctx: *mut JSContext,
    mut func: Option<JSCFunction>,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
) -> JSValue {
    return JS_NewCFunction2(ctx, func, name, length, JS_CFUNC_generic, 0 as libc::c_int);
}
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
pub unsafe extern "C" fn JS_NewCFunctionLen(
    mut ctx: *mut JSContext,
    mut func: Option<JSCFunction>,
    mut name: *const libc::c_char,
    mut name_length: libc::c_int,
    mut length: libc::c_int,
) -> JSValue {
    return JS_NewCFunction2Len(
        ctx,
        func,
        name,
        name_length,
        length,
        JS_CFUNC_generic,
        0 as libc::c_int,
    );
}
#[inline]
pub unsafe extern "C" fn JS_NewCFunctionMagic(
    mut ctx: *mut JSContext,
    mut func: Option<JSCFunctionMagic>,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
    mut cproto: JSCFunctionEnum,
    mut magic: libc::c_int,
) -> JSValue {
    let mut ft: JSCFunctionType = JSCFunctionType {
        generic_magic: func,
    };
    return JS_NewCFunction2(ctx, ft.generic, name, length, cproto, magic);
}
#[inline]
pub unsafe extern "C" fn JS_GetTag(mut v: JSValue) -> int64_t {
    return v.tag as int32_t as int64_t;
}
#[inline]
pub unsafe extern "C" fn JS_GetInt(mut v: JSValue) -> int32_t {
    return v.u.int32;
}
#[inline]
pub unsafe extern "C" fn JS_GetBool(mut v: JSValue) -> int32_t {
    return v.u.int32;
}
#[inline]
pub unsafe extern "C" fn JS_GetFloat64(mut v: JSValue) -> libc::c_double {
    return v.u.float64;
}
#[inline]
pub unsafe extern "C" fn JS_GetPtr(mut v: JSValue) -> *mut libc::c_void {
    return v.u.ptr;
}
#[inline]
pub unsafe extern "C" fn JS_MakeValue(mut tag: int32_t, mut val: int32_t) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion { int32: val },
            tag: tag as int64_t,
        };
        init
    };
}
#[inline]
pub unsafe extern "C" fn JS_MakePtr(mut tag: int32_t, mut p: uintptr_t) -> JSValue {
    return {
        let mut init = JSValue {
            u: JSValueUnion {
                ptr: p as *mut libc::c_void,
            },
            tag: tag as int64_t,
        };
        init
    };
}
#[inline]
pub unsafe extern "C" fn JS_MakeNAN() -> JSValue {
    return JS_NAN;
}
#[inline]
pub unsafe extern "C" fn JS_MakeNULL() -> JSValue {
    return JS_NULL;
}
#[inline]
pub unsafe extern "C" fn JS_MakeUNDEFINED() -> JSValue {
    return JS_UNDEFINED;
}
#[inline]
pub unsafe extern "C" fn JS_MakeFALSE() -> JSValue {
    return JS_FALSE;
}
#[inline]
pub unsafe extern "C" fn JS_MakeTRUE() -> JSValue {
    return JS_TRUE;
}
#[inline]
pub unsafe extern "C" fn JS_MakeEXCEPTION() -> JSValue {
    return JS_EXCEPTION;
}
#[inline]
pub unsafe extern "C" fn JS_MakeUNINITIALIZED() -> JSValue {
    return JS_UNINITIALIZED;
}
#[inline]
pub unsafe extern "C" fn _to_keep_symbols(mut flags: libc::c_int) -> libc::c_int {
    return (flags & JS_PROP_CONFIGURABLE & JS_PROP_WRITABLE & JS_PROP_ENUMERABLE != 0
        && JS_PROP_LENGTH
            & JS_PROP_TMASK
            & JS_PROP_NORMAL
            & JS_PROP_GETSET
            & JS_PROP_VARREF
            & JS_PROP_AUTOINIT
            != 0) as libc::c_int;
}
