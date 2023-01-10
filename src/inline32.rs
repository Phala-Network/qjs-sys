#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables
)]

use crate::c::*;
use crate::libc;

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
pub type JSValue = uint64_t;
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
pub unsafe extern "C" fn JS_NewCFunction(
    mut ctx: *mut JSContext,
    mut func: Option<JSCFunction>,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
) -> JSValue {
    return JS_NewCFunction2(ctx, func, name, length, JS_CFUNC_generic, 0 as libc::c_int);
}
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
pub unsafe extern "C" fn __JS_NewFloat64(
    mut ctx: *mut JSContext,
    mut d: libc::c_double,
) -> JSValue {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { d: 0. };
    let mut v: JSValue = 0;
    u.d = d;
    if (u.u64_0 & 0x7fffffffffffffff as libc::c_longlong as libc::c_ulonglong
        > 0x7ff0000000000000 as libc::c_longlong as libc::c_ulonglong) as libc::c_int
        as libc::c_long
        != 0
    {
        v = (0x7ff8000000000000 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
    } else {
        v = (u.u64_0).wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_VALUE_GET_NORM_TAG(mut v: JSValue) -> libc::c_int {
    let mut tag: uint32_t = 0;
    tag = (v >> 32 as libc::c_int) as libc::c_int as uint32_t;
    if tag.wrapping_sub(JS_TAG_FIRST as libc::c_int as libc::c_uint)
        >= (JS_TAG_FLOAT64 as libc::c_int - JS_TAG_FIRST as libc::c_int) as libc::c_uint
    {
        return JS_TAG_FLOAT64 as libc::c_int;
    } else {
        return tag as libc::c_int;
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
    if val == val as int32_t as libc::c_longlong {
        v = JS_NewInt32(ctx, val as int32_t);
    } else {
        v = __JS_NewFloat64(ctx, val as libc::c_double);
    }
    return v;
}
#[inline(always)]
pub unsafe extern "C" fn JS_NewUint32(mut ctx: *mut JSContext, mut val: uint32_t) -> JSValue {
    let mut v: JSValue = 0;
    if val <= 0x7fffffff as libc::c_int as libc::c_uint {
        v = JS_NewInt32(ctx, val as int32_t);
    } else {
        v = __JS_NewFloat64(ctx, val as libc::c_double);
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
    if u.u == t.u {
        v = (JS_TAG_INT as libc::c_int as uint64_t) << 32 as libc::c_int
            | val as uint32_t as libc::c_ulonglong;
    } else {
        v = __JS_NewFloat64(ctx, d);
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
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        let mut p: *mut JSRefCountHeader =
            v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
        let ref mut fresh0 = (*p).ref_count;
        *fresh0 -= 1;
        if *fresh0 <= 0 as libc::c_int {
            __JS_FreeValue(ctx, v);
        }
    }
}
#[inline]
pub unsafe extern "C" fn JS_FreeValueRT(mut rt: *mut JSRuntime, mut v: JSValue) {
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        let mut p: *mut JSRefCountHeader =
            v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
        let ref mut fresh1 = (*p).ref_count;
        *fresh1 -= 1;
        if *fresh1 <= 0 as libc::c_int {
            __JS_FreeValueRT(rt, v);
        }
    }
}
#[inline]
pub unsafe extern "C" fn JS_DupValue(mut ctx: *mut JSContext, mut v: JSValue) -> JSValue {
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        let mut p: *mut JSRefCountHeader =
            v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
        let ref mut fresh2 = (*p).ref_count;
        *fresh2 += 1;
    }
    return v;
}
#[inline]
pub unsafe extern "C" fn JS_DupValueRT(mut rt: *mut JSRuntime, mut v: JSValue) -> JSValue {
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        let mut p: *mut JSRefCountHeader =
            v as intptr_t as *mut libc::c_void as *mut JSRefCountHeader;
        let ref mut fresh3 = (*p).ref_count;
        *fresh3 += 1;
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
pub const JS_PROP_THROW: libc::c_int = (1 as libc::c_int) << 14 as libc::c_int;

#[inline]
pub fn JS_GetTag(mut v: JSValue) -> int32_t {
    return (v >> 32 as libc::c_int) as int32_t;
}
#[inline]
pub fn JS_GetInt(mut v: JSValue) -> int32_t {
    return v as libc::c_int;
}
#[inline]
pub fn JS_GetBool(mut v: JSValue) -> int32_t {
    return v as libc::c_int;
}
#[inline]
pub fn JS_GetFloat64(mut v: JSValue) -> libc::c_double {
    return unsafe { JS_VALUE_GET_FLOAT64(v) };
}
#[inline]
pub fn JS_GetPtr(mut v: JSValue) -> *mut libc::c_void {
    return v as intptr_t as *mut libc::c_void;
}

#[inline]
pub const fn JS_MakeValue(mut tag: int32_t, mut val: int32_t) -> JSValue {
    return (tag as uint64_t) << 32 as libc::c_int | val as uint32_t as libc::c_ulonglong;
}

#[inline]
pub const fn JS_MakePtr(mut tag: int32_t, mut p: uintptr_t) -> JSValue {
    return (tag as uint64_t) << 32 as libc::c_int | p as libc::c_ulonglong;
}

#[inline]
pub const fn JS_MakeNAN() -> JSValue {
    return (0x7ff8000000000000 as libc::c_longlong as libc::c_ulonglong)
        .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
}
#[inline]
pub const fn JS_MakeNULL() -> JSValue {
    return JS_NULL;
}
const JS_NULL: libc::c_ulonglong = (JS_TAG_NULL as libc::c_int as uint64_t) << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
#[inline]
pub const fn JS_MakeUNDEFINED() -> JSValue {
    return JS_UNDEFINED;
}
const JS_UNDEFINED: libc::c_ulonglong = (JS_TAG_UNDEFINED as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
#[inline]
pub const fn JS_MakeFALSE() -> JSValue {
    return JS_FALSE;
}
const JS_FALSE: libc::c_ulonglong = (JS_TAG_BOOL as libc::c_int as uint64_t) << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
#[inline]
pub const fn JS_MakeTRUE() -> JSValue {
    return JS_TRUE;
}
const JS_TRUE: libc::c_ulonglong = (JS_TAG_BOOL as libc::c_int as uint64_t) << 32 as libc::c_int
    | 1 as libc::c_int as uint32_t as libc::c_ulonglong;
#[inline]
pub const fn JS_MakeEXCEPTION() -> JSValue {
    return JS_EXCEPTION;
}
const JS_EXCEPTION: libc::c_ulonglong = (JS_TAG_EXCEPTION as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
#[inline]
pub const fn JS_MakeUNINITIALIZED() -> JSValue {
    return JS_UNINITIALIZED;
}
const JS_UNINITIALIZED: libc::c_ulonglong = (JS_TAG_UNINITIALIZED as libc::c_int as uint64_t)
    << 32 as libc::c_int
    | 0 as libc::c_int as uint32_t as libc::c_ulonglong;

#[inline]
pub const fn JS_MakeFloat64(mut d: libc::c_double) -> JSValue {
    unsafe {
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
                v = (u.u64_0)
                    .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
            }
            _ => {
                v = (0x7ff8000000000000 as libc::c_longlong as libc::c_ulonglong)
                    .wrapping_sub((JS_FLOAT64_TAG_ADDEND as uint64_t) << 32 as libc::c_int);
            }
        }
        return v;
    }
}
