use ::libc;
extern "C" {
    pub type JSContext;
    fn __JS_FreeValue(ctx: *mut JSContext, v: JSValue);
    fn JS_ToCStringLen2(
        ctx: *mut JSContext,
        plen: *mut size_t,
        val1: JSValue,
        cesu8: libc::c_int,
    ) -> *const libc::c_char;
    fn JS_FreeCString(ctx: *mut JSContext, ptr: *const libc::c_char);
    fn JS_NewObject(ctx: *mut JSContext) -> JSValue;
    fn JS_SetPropertyStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const libc::c_char,
        val: JSValue,
    ) -> libc::c_int;
    fn JS_GetGlobalObject(ctx: *mut JSContext) -> JSValue;
    fn JS_NewCFunction2(
        ctx: *mut JSContext,
        func: Option::<JSCFunction>,
        name: *const libc::c_char,
        length: libc::c_int,
        cproto: JSCFunctionEnum,
        magic: libc::c_int,
    ) -> JSValue;
}
pub type size_t = libc::c_ulong;
pub type intptr_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
pub type JSCFunction = unsafe extern "C" fn(
    *mut JSContext,
    JSValue,
    libc::c_int,
    *mut JSValue,
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
#[inline]
unsafe extern "C" fn JS_FreeValue(mut ctx: *mut JSContext, mut v: JSValue) {
    if (v >> 32 as libc::c_int) as libc::c_int as libc::c_uint
        >= JS_TAG_FIRST as libc::c_int as libc::c_uint
    {
        let mut p: *mut JSRefCountHeader = v as intptr_t as *mut libc::c_void
            as *mut JSRefCountHeader;
        let ref mut fresh0 = (*p).ref_count;
        *fresh0 -= 1;
        if *fresh0 <= 0 as libc::c_int {
            __JS_FreeValue(ctx, v);
        }
    }
}
#[inline]
unsafe extern "C" fn JS_ToCStringLen(
    mut ctx: *mut JSContext,
    mut plen: *mut size_t,
    mut val1: JSValue,
) -> *const libc::c_char {
    return JS_ToCStringLen2(ctx, plen, val1, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn JS_NewCFunction(
    mut ctx: *mut JSContext,
    mut func: Option::<JSCFunction>,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
) -> JSValue {
    return JS_NewCFunction2(ctx, func, name, length, JS_CFUNC_generic, 0 as libc::c_int);
}
unsafe extern "C" fn js_print(
    mut ctx: *mut JSContext,
    mut this_val: JSValue,
    mut argc: libc::c_int,
    mut argv: *mut JSValue,
) -> JSValue {
    let mut i: libc::c_int = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    i = 0 as libc::c_int;
    while i < argc {
        if i != 0 as libc::c_int {
            putchar(' ' as i32);
        }
        str = JS_ToCStringLen(ctx, &mut len, *argv.offset(i as isize));
        if str.is_null() {
            return (JS_TAG_EXCEPTION as libc::c_int as uint64_t) << 32 as libc::c_int
                | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
        }
        JS_FreeCString(ctx, str);
        i += 1;
    }
    putchar('\n' as i32);
    return (JS_TAG_UNDEFINED as libc::c_int as uint64_t) << 32 as libc::c_int
        | 0 as libc::c_int as uint32_t as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn js_env_add_helpers(mut ctx: *mut JSContext) {
    let mut global_obj: JSValue = 0;
    let mut console: JSValue = 0;
    let mut args: JSValue = 0;
    global_obj = JS_GetGlobalObject(ctx);
    console = JS_NewObject(ctx);
    JS_SetPropertyStr(
        ctx,
        console,
        b"log\0" as *const u8 as *const libc::c_char,
        JS_NewCFunction(
            ctx,
            Some(
                js_print
                    as unsafe extern "C" fn(
                        *mut JSContext,
                        JSValue,
                        libc::c_int,
                        *mut JSValue,
                    ) -> JSValue,
            ),
            b"log\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        ),
    );
    JS_SetPropertyStr(
        ctx,
        console,
        b"error\0" as *const u8 as *const libc::c_char,
        JS_NewCFunction(
            ctx,
            Some(
                js_print
                    as unsafe extern "C" fn(
                        *mut JSContext,
                        JSValue,
                        libc::c_int,
                        *mut JSValue,
                    ) -> JSValue,
            ),
            b"error\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        ),
    );
    JS_SetPropertyStr(
        ctx,
        global_obj,
        b"console\0" as *const u8 as *const libc::c_char,
        console,
    );
    JS_SetPropertyStr(
        ctx,
        global_obj,
        b"print\0" as *const u8 as *const libc::c_char,
        JS_NewCFunction(
            ctx,
            Some(
                js_print
                    as unsafe extern "C" fn(
                        *mut JSContext,
                        JSValue,
                        libc::c_int,
                        *mut JSValue,
                    ) -> JSValue,
            ),
            b"print\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        ),
    );
    JS_FreeValue(ctx, global_obj);
}
