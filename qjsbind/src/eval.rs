use core::{ffi::CStr, ptr::NonNull};

use alloc::string::{String, ToString};

use crate::traits::ToNonNull;
use crate::{c, Value};

pub enum JsCode<'a> {
    Source(&'a str),
    Bytecode(&'a [u8]),
}

pub fn eval(ctx: NonNull<c::JSContext>, script: &JsCode) -> Result<Value, String> {
    struct IO {
        output: Result<Value, String>,
    }

    unsafe extern "C" fn output(
        ctx: *mut c::JSContext,
        userdata: *mut ::core::ffi::c_void,
        output: c::JSValueConst,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        io.output = Ok(Value::new_cloned(
            ctx.to_non_null().expect("output with null contect"),
            output,
        ));
    }

    unsafe extern "C" fn output_err(
        _ctx: *mut c::JSContext,
        userdata: *mut ::core::ffi::c_void,
        err: *const ::core::ffi::c_char,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        let err = unsafe { CStr::from_ptr(err) };
        io.output = Err(err.to_string_lossy().into_owned());
    }

    let mut userdata = IO {
        output: Ok(Value::undefined()),
    };

    let mut callbacks = c::callbacks_t {
        userdata: &mut userdata as *mut _ as *mut ::core::ffi::c_void,
        output: Some(output),
        output_err: Some(output_err),
        read_args: None,
    };

    let code = match script {
        JsCode::Source(src) => c::code_t {
            code: src.as_ptr() as _,
            code_len: src.as_bytes().len() as _,
            is_bytecode: 0,
        },
        JsCode::Bytecode(bytes) => c::code_t {
            code: bytes.as_ptr() as _,
            code_len: bytes.len() as _,
            is_bytecode: 1,
        },
    };

    let ret = unsafe { c::js_eval_code(ctx.as_ptr(), &code, &mut callbacks) };
    if ret == 0 {
        userdata.output
    } else {
        let output = userdata.output?;
        Err(output.to_string())
    }
}
