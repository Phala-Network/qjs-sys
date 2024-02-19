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
        obj: JSValue,
        prop: JSAtom,
        val: JSValue,
        this_obj: JSValue,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn JS_NewCFunction2Len(
        ctx: *mut JSContext,
        func: Option<JSCFunction>,
        name: *const libc::c_char,
        name_length: libc::c_int,
        length: libc::c_int,
        cproto: JSCFunctionEnum,
        magic: libc::c_int,
    ) -> JSValue;
    fn JS_NewCFunction2(
        ctx: *mut JSContext,
        func: Option<JSCFunction>,
        name: *const libc::c_char,
        length: libc::c_int,
        cproto: JSCFunctionEnum,
        magic: libc::c_int,
    ) -> JSValue;
}
pub type size_t = usize;
pub type uintptr_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
pub type int32_t = libc::c_int;
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
pub const JS_TAG_BIG_FLOAT: C2RustUnnamed = -9;
pub const JS_TAG_BIG_INT: C2RustUnnamed = -10;
pub const JS_TAG_BIG_DECIMAL: C2RustUnnamed = -11;
pub const JS_TAG_FIRST: C2RustUnnamed = -11;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSRefCountHeader {
    pub ref_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub v: JSValue,
    pub d: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub d: libc::c_double,
    pub u: uint64_t,
}
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
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
pub unsafe extern "C" fn JS_VALUE_GET_FLOAT64(mut v: JSValue) -> libc::c_double {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { v: 0 };
    u.v = v;
    u.v = (u.v as libc::c_ulonglong)
        .wrapping_add((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int) as JSValue
        as JSValue;
    return u.d;
}
pub const JS_FLOAT64_TAG_ADDEND: libc::c_int =
    0x7ff80000 as libc::c_int - JS_TAG_FIRST as libc::c_int + 1 as libc::c_int;
#[inline]
pub unsafe extern "C" fn JS_MakeFloat64(mut d: libc::c_double) -> JSValue {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { d: 0. };
    let mut v: JSValue = 0;
    u.d = d;
    let mut current_block_2: u64;
    if (u.u64_0 & 0x7fffffffffffffff as libc::c_longlong as libc::c_ulonglong
        > 0x7ff0000000000000 as libc::c_longlong as libc::c_ulonglong) as libc::c_int
        as libc::c_long
        != 0
    {
        current_block_2 = 4988723283678924448;
    } else {
        current_block_2 = 14916268686031723178;
    }
    match current_block_2 {
        14916268686031723178 => {
            v = (u.u64_0).wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
        }
        _ => {
            v = (0x7ff8000000000000 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
        }
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_VALUE_GET_NORM_TAG(mut v: JSValue) -> libc::c_int {
    let mut tag: uint32_t = 0;
    tag = (v >> 32 as libc::c_int) as libc::c_int as uint32_t;
    let mut current_block_2: u64;
    if tag.wrapping_sub(JS_TAG_FIRST as libc::c_int as libc::c_uint)
        >= (JS_TAG_FLOAT64 as libc::c_int - JS_TAG_FIRST as libc::c_int) as libc::c_uint
    {
        current_block_2 = 6239978542346980191;
    } else {
        current_block_2 = 16559507199688588974;
    }
    match current_block_2 {
        16559507199688588974 => return tag as libc::c_int,
        _ => return JS_TAG_FLOAT64 as libc::c_int,
    };
}
#[inline]
pub unsafe extern "C" fn JS_VALUE_IS_NAN(mut v: JSValue) -> libc::c_int {
    let mut tag: uint32_t = 0;
    tag = (v >> 32 as libc::c_int) as libc::c_int as uint32_t;
    return (tag as libc::c_ulonglong
        == (0x7ff8000000000000 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int)
            >> 32 as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn __JS_NewFloat64(
    mut ctx: *mut JSContext,
    mut d: libc::c_double,
) -> JSValue {
    return JS_MakeFloat64(d);
}
pub const JS_PROP_CONFIGURABLE: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const JS_PROP_WRITABLE: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
pub const JS_PROP_ENUMERABLE: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
#[inline(always)]
pub unsafe extern "C" fn JS_NewBool(mut ctx: *mut JSContext, mut val: libc::c_int) -> JSValue {
    return (JS_TAG_BOOL as libc::c_int as uint64_t) << 32 as libc::c_int
        | (val != 0 as libc::c_int) as libc::c_int as uint32_t as libc::c_ulonglong;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewInt32(mut ctx: *mut JSContext, mut val: int32_t) -> JSValue {
    return (JS_TAG_INT as libc::c_int as uint64_t) << 32 as libc::c_int
        | val as uint32_t as libc::c_ulonglong;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewCatchOffset(mut ctx: *mut JSContext, mut val: int32_t) -> JSValue {
    return (JS_TAG_CATCH_OFFSET as libc::c_int as uint64_t) << 32 as libc::c_int
        | val as uint32_t as libc::c_ulonglong;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewInt64(mut ctx: *mut JSContext, mut val: int64_t) -> JSValue {
    let mut v: JSValue = 0;
    let mut current_block_3: u64;
    if val == val as int32_t as libc::c_longlong {
        current_block_3 = 11174649648027449784;
    } else {
        current_block_3 = 10771263883588581193;
    }
    match current_block_3 {
        10771263883588581193 => {
            v = __JS_NewFloat64(ctx, val as libc::c_double);
        }
        _ => {
            v = JS_NewInt32(ctx, val as int32_t);
        }
    }
    return v;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewUint32(mut ctx: *mut JSContext, mut val: uint32_t) -> JSValue {
    let mut v: JSValue = 0;
    let mut current_block_3: u64;
    if val <= 0x7fffffff as libc::c_int as libc::c_uint {
        current_block_3 = 11174649648027449784;
    } else {
        current_block_3 = 10771263883588581193;
    }
    match current_block_3 {
        10771263883588581193 => {
            v = __JS_NewFloat64(ctx, val as libc::c_double);
        }
        _ => {
            v = JS_NewInt32(ctx, val as int32_t);
        }
    }
    return v;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewFloat64(mut ctx: *mut JSContext, mut d: libc::c_double) -> JSValue {
    let mut v: JSValue = 0;
    let mut val: int32_t = 0;
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { d: 0. };
    let mut t: C2RustUnnamed_2 = C2RustUnnamed_2 { d: 0. };
    u.d = d;
    val = d as int32_t;
    t.d = val as libc::c_double;
    let mut current_block_6: u64;
    if u.u == t.u {
        current_block_6 = 735147466149431745;
    } else {
        current_block_6 = 7502529970979898288;
    }
    match current_block_6 {
        7502529970979898288 => {
            v = __JS_NewFloat64(ctx, d);
        }
        _ => {
            v = (JS_TAG_INT as libc::c_int as uint64_t) << 32 as libc::c_int
                | val as uint32_t as libc::c_ulonglong;
        }
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_IsNumber(mut v: JSValue) -> libc::c_int {
    let mut tag: libc::c_int = (v >> 32 as libc::c_int) as libc::c_int;
    return (tag == JS_TAG_INT as libc::c_int
        || (tag - JS_TAG_FIRST as libc::c_int) as libc::c_uint
            >= (JS_TAG_FLOAT64 as libc::c_int - JS_TAG_FIRST as libc::c_int) as libc::c_uint)
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsBigInt(mut v: JSValue) -> libc::c_int {
    let mut tag: libc::c_int = (v >> 32 as libc::c_int) as libc::c_int;
    return (tag == JS_TAG_BIG_INT as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsBigFloat(mut v: JSValue) -> libc::c_int {
    let mut tag: libc::c_int = (v >> 32 as libc::c_int) as libc::c_int;
    return (tag == JS_TAG_BIG_FLOAT as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsBigDecimal(mut v: JSValue) -> libc::c_int {
    let mut tag: libc::c_int = (v >> 32 as libc::c_int) as libc::c_int;
    return (tag == JS_TAG_BIG_DECIMAL as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsBool(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_BOOL as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsNull(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_NULL as libc::c_int) as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsUndefined(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_UNDEFINED as libc::c_int)
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsException(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_EXCEPTION as libc::c_int)
        as libc::c_int as libc::c_long as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsUninitialized(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_UNINITIALIZED as libc::c_int)
        as libc::c_int as libc::c_long as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsString(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_STRING as libc::c_int)
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsSymbol(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_SYMBOL as libc::c_int)
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_IsObject(mut v: JSValue) -> libc::c_int {
    return ((v >> 32 as libc::c_int) as libc::c_int == JS_TAG_OBJECT as libc::c_int)
        as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_FreeValue(mut ctx: *mut JSContext, mut v: JSValue) {
    let mut current_block_3: u64;
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        current_block_3 = 9188997750422900590;
    } else {
        current_block_3 = 7502529970979898288;
    }
    match current_block_3 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader =
                v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
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
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        current_block_3 = 9188997750422900590;
    } else {
        current_block_3 = 7502529970979898288;
    }
    match current_block_3 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader =
                v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
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
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        current_block_1 = 9188997750422900590;
    } else {
        current_block_1 = 14155750587950065367;
    }
    match current_block_1 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader =
                v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
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
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        current_block_1 = 9188997750422900590;
    } else {
        current_block_1 = 14155750587950065367;
    }
    match current_block_1 {
        9188997750422900590 => {
            let mut p: *mut JSRefCountHeader =
                v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
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
pub const JS_PROP_LENGTH: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const JS_PROP_TMASK: libc::c_int = (3 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_NORMAL: libc::c_int = (0 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_GETSET: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_VARREF: libc::c_int = (2 as libc::c_int) << 4 as libc::c_int;
pub const JS_PROP_AUTOINIT: libc::c_int = (3 as libc::c_int) << 4 as libc::c_int;
pub const JS_UNINITIALIZED: libc::c_ulonglong = (JS_TAG_UNINITIALIZED as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
pub const JS_EXCEPTION: libc::c_ulonglong = (JS_TAG_EXCEPTION as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
pub const JS_TRUE: libc::c_ulonglong = (JS_TAG_BOOL as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 1 as libc::c_int as uint32_t as libc::c_ulonglong;
pub const JS_FALSE: libc::c_ulonglong = (JS_TAG_BOOL as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
pub const JS_UNDEFINED: libc::c_ulonglong = (JS_TAG_UNDEFINED as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
pub const JS_NULL: libc::c_ulonglong = (JS_TAG_NULL as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
pub const JS_PROP_THROW: libc::c_int = (1 as libc::c_int) << 14 as libc::c_int;
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
    return JS_SetPropertyInternal(ctx, this_obj, prop, val, this_obj, JS_PROP_THROW);
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
    return JS_NewCFunction2(
        ctx,
        ::core::mem::transmute::<Option<JSCFunctionMagic>, Option<JSCFunction>>(func),
        name,
        length,
        cproto,
        magic,
    );
}
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
pub unsafe extern "C" fn JS_NewCFunction(
    mut ctx: *mut JSContext,
    mut func: Option<JSCFunction>,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
) -> JSValue {
    return JS_NewCFunction2(ctx, func, name, length, JS_CFUNC_generic, 0 as libc::c_int);
}
#[inline]
pub unsafe extern "C" fn JS_GetTag(mut v: JSValue) -> int64_t {
    return (v >> 32 as libc::c_int) as libc::c_int as int64_t;
}
#[inline]
pub unsafe extern "C" fn JS_GetInt(mut v: JSValue) -> int32_t {
    return v as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_GetBool(mut v: JSValue) -> int32_t {
    return v as libc::c_int;
}
#[inline]
pub unsafe extern "C" fn JS_GetFloat64(mut v: JSValue) -> libc::c_double {
    return JS_VALUE_GET_FLOAT64(v);
}
#[inline]
pub unsafe extern "C" fn JS_GetPtr(mut v: JSValue) -> *mut libc::c_void {
    return v as intptr_t as *mut libc::c_void;
}
#[inline]
pub unsafe extern "C" fn JS_MakeValue(mut tag: int32_t, mut val: int32_t) -> JSValue {
    return (tag as uint64_t) << 32 as libc::c_int | val as uint32_t as libc::c_ulonglong;
}
#[inline]
pub unsafe extern "C" fn JS_MakePtr(mut tag: int32_t, mut p: uintptr_t) -> JSValue {
    return (tag as uint64_t) << 32 as libc::c_int | p as libc::c_ulonglong;
}
#[inline]
pub unsafe extern "C" fn JS_MakeNAN() -> JSValue {
    return (0x7ff8000000000000 as libc::c_longlong as libc::c_ulonglong)
        .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
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
