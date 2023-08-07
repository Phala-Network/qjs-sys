#![no_std]
extern crate alloc;

use core::ffi::CStr;

use alloc::ffi::CString;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

mod alloc_impl;

pub mod c;
pub mod convert;
#[cfg_attr(target_pointer_width = "32", path = "inline32.rs")]
#[cfg_attr(target_pointer_width = "64", path = "inline64.rs")]
pub mod inline_fns;
mod libc {
    pub use core::ffi::*;
}

pub fn throw_type_error(ctx: *mut c::JSContext, msg: &str) -> c::JSValue {
    let cmsg = CString::new(msg).unwrap_or_default();
    unsafe { c::JS_ThrowTypeError(ctx, cmsg.as_ptr()) }
}

pub enum JsCode<'a> {
    Source(&'a CStr),
    Bytecode(&'a [u8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    String(String),
    Bytes(Vec<u8>),
    Undefined,
}

pub fn eval(scripts: &[JsCode], args: &[String]) -> Result<Output, String> {
    struct IO<'a> {
        args: &'a [String],
        output: Option<Output>,
    }

    extern "C" fn read_args(
        userdata: *mut ::core::ffi::c_void,
        eng_userdata: *mut ::core::ffi::c_void,
        handler: c::input_handler_t,
    ) -> ::core::ffi::c_int {
        let io = unsafe { &mut *(userdata as *mut IO) };

        for (i, arg) in io.args.iter().enumerate() {
            let handler = handler.unwrap();
            let ret = unsafe { handler(eng_userdata, i as _, arg.as_ptr() as _, arg.len() as _) };
            if ret != 0 {
                return ret;
            }
        }
        0
    }

    unsafe extern "C" fn output_bytes(
        userdata: *mut ::core::ffi::c_void,
        output: *const ::core::ffi::c_char,
        output_len: ::core::ffi::c_int,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        let bytes: &[u8] = unsafe { core::slice::from_raw_parts(output as _, output_len as _) };
        io.output = Some(Output::Bytes(bytes.to_vec()));
    }

    unsafe extern "C" fn output_str(
        userdata: *mut ::core::ffi::c_void,
        output: *const ::core::ffi::c_char,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        let cstr = unsafe { CStr::from_ptr(output) };
        let s = cstr.to_str().unwrap_or("<Invalid UTF8 sequnece>");
        io.output = Some(Output::String(s.to_string()));
    }

    let mut userdata = IO { args, output: None };

    let mut callbacks = c::callbacks_t {
        userdata: &mut userdata as *mut _ as *mut ::core::ffi::c_void,
        output_str: Some(output_str),
        output_bytes: Some(output_bytes),
        read_args: Some(read_args),
    };

    let codes: Vec<_> = scripts
        .into_iter()
        .map(|s| match s {
            JsCode::Source(src) => c::code_t {
                code: src.as_ptr() as _,
                code_len: src.to_bytes().len() as _,
                is_bytecode: 0,
            },
            JsCode::Bytecode(bytes) => c::code_t {
                code: bytes.as_ptr() as _,
                code_len: bytes.len() as _,
                is_bytecode: 1,
            },
        })
        .collect();

    let ret = unsafe { c::js_eval(codes.as_ptr() as _, codes.len() as _, &mut callbacks) };
    if ret == 0 {
        let output = match userdata.output {
            Some(output) => output,
            None => Output::Undefined,
        };
        Ok(output)
    } else {
        let output = match userdata.output {
            Some(Output::String(s)) => s,
            _ => "UnknownError".to_string(),
        };
        Err(output)
    }
}

pub fn ctx_eval(ctx: *mut c::JSContext, script: JsCode) -> Result<Output, String> {
    struct IO {
        output: Option<Output>,
    }

    unsafe extern "C" fn output_bytes(
        userdata: *mut ::core::ffi::c_void,
        output: *const ::core::ffi::c_char,
        output_len: ::core::ffi::c_int,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        let bytes: &[u8] = unsafe { core::slice::from_raw_parts(output as _, output_len as _) };
        io.output = Some(Output::Bytes(bytes.to_vec()));
    }

    unsafe extern "C" fn output_str(
        userdata: *mut ::core::ffi::c_void,
        output: *const ::core::ffi::c_char,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        let cstr = unsafe { CStr::from_ptr(output) };
        let s = cstr.to_str().unwrap_or("<Invalid UTF8 sequnece>");
        io.output = Some(Output::String(s.to_string()));
    }

    let mut userdata = IO { output: None };

    let mut callbacks = c::callbacks_t {
        userdata: &mut userdata as *mut _ as *mut ::core::ffi::c_void,
        output_str: Some(output_str),
        output_bytes: Some(output_bytes),
        read_args: None,
    };

    let code = match script {
        JsCode::Source(src) => c::code_t {
            code: src.as_ptr() as _,
            code_len: src.to_bytes().len() as _,
            is_bytecode: 0,
        },
        JsCode::Bytecode(bytes) => c::code_t {
            code: bytes.as_ptr() as _,
            code_len: bytes.len() as _,
            is_bytecode: 1,
        },
    };

    let ret = unsafe { c::js_eval_code(ctx, &code, &mut callbacks) };
    if ret == 0 {
        let output = match userdata.output {
            Some(output) => output,
            None => Output::Undefined,
        };
        Ok(output)
    } else {
        let output = match userdata.output {
            Some(Output::String(s)) => s,
            _ => "UnknownError".to_string(),
        };
        Err(output)
    }
}

pub fn ctx_get_exception_str(ctx: *mut c::JSContext) -> String {
    unsafe {
        let mut len: c::size_t = 0;
        let e = c::JS_GetException(ctx);
        let ptr = c::JS_ToCStringLen(ctx, &mut len, e);
        let bytes: &[u8] = core::slice::from_raw_parts(ptr as _, len as _);
        let s = String::from_utf8_lossy(bytes).into_owned();
        c::JS_FreeCString(ctx, ptr as _);
        s
    }
}

pub fn ctx_to_string(ctx: *mut c::JSContext, value: c::JSValueConst) -> String {
    let mut len: c::size_t = 0;
    let ptr = unsafe { c::JS_ToCStringLen(ctx, &mut len, value) };
    let bytes: &[u8] = unsafe { core::slice::from_raw_parts(ptr as _, len as _) };
    let s = String::from_utf8_lossy(bytes).into_owned();
    unsafe { c::JS_FreeCString(ctx, ptr as _) };
    s
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
