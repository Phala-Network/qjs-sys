use alloc::ffi::CString;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::c;
use crate as js;

pub fn js_throw_type_error(ctx: &js::Context, msg: &str) -> c::JSValue {
    let cmsg = CString::new(msg).unwrap_or_default();
    unsafe { c::JS_ThrowTypeError(ctx.as_ptr(), cmsg.as_ptr()) }
}

pub fn ctx_to_str<T>(
    ctx: &js::Context,
    value: c::JSValueConst,
    cb: impl FnOnce(&str) -> T,
) -> T {
    unsafe {
        let mut len: c::size_t = 0;
        let ptr = c::JS_ToCStringLen(ctx.as_ptr(), &mut len, value);
        if ptr.is_null() {
            return cb("");
        }
        let bytes: &[u8] = core::slice::from_raw_parts(ptr as _, len as _);
        let s = core::str::from_utf8_unchecked(bytes);
        let rv = cb(s);
        c::JS_FreeCString(ctx.as_ptr(), ptr as _);
        rv
    }
}

pub fn ctx_to_string(ctx: &js::Context, value: c::JSValueConst) -> String {
    ctx_to_str(ctx, value, |s| s.to_string())
}

pub fn compile(code: &str, filename: &str) -> Result<Vec<u8>, &'static str> {
    use crate::c as js;
    let code = CString::new(code).or(Err("Invalid encoding in js code"))?;
    let filename = CString::new(filename).or(Err("Invalid filename"))?;
    unsafe {
        let rt = js::JS_NewRuntime();
        if rt.is_null() {
            return Err("Failed to create js runtime");
        }
        scopeguard::defer! {
            js::JS_FreeRuntime(rt);
        }

        let ctx = js::JS_NewContext(rt);
        if ctx.is_null() {
            return Err("Failed to create js context");
        }
        scopeguard::defer! {
            js::JS_FreeContext(ctx);
        }

        let bytecode = js::JS_Eval(
            ctx,
            code.as_ptr() as _,
            code.to_bytes().len() as _,
            filename.as_ptr() as _,
            js::JS_EVAL_FLAG_COMPILE_ONLY as _,
        );

        if js::JS_IsException(bytecode) != 0 {
            c::js_std_dump_error(ctx);
            return Err("Failed to compile js code");
        }
        scopeguard::defer! {
            js::JS_FreeValue(ctx, bytecode);
        }

        let flags = js::JS_WRITE_OBJ_BYTECODE;
        let mut out_buf_len = 0;
        let out_buf = js::JS_WriteObject(ctx, &mut out_buf_len, bytecode, flags as _);

        if out_buf.is_null() {
            return Err("Failed to dump bytecode");
        }
        scopeguard::defer! {
            js::js_free(ctx, out_buf as _);
        }

        let bytes = core::slice::from_raw_parts(out_buf as *const u8, out_buf_len as _).to_vec();
        Ok(bytes)
    }
}
