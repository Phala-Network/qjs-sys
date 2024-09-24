use core::ffi::CStr;

use alloc::string::{String, ToString};

use crate::{self as js, c, Value};

pub enum Code<'a> {
    Source(&'a str),
    Bytecode(&'a [u8]),
}

pub fn eval(ctx: &js::Context, script: &Code) -> Result<Value, String> {
    struct IO {
        output: Result<Value, String>,
    }

    unsafe extern "C" fn output(
        ctx: *mut c::JSContext,
        userdata: *mut ::core::ffi::c_void,
        output: c::JSValueConst,
    ) {
        let io = unsafe { &mut *(userdata as *mut IO) };
        let ctx = js::Context::clone_from_ptr(ctx).expect("output with null contect");
        io.output = Ok(Value::new_cloned(&ctx, output));
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
        Code::Source(src) => c::code_t {
            code: src.as_ptr() as _,
            code_len: src.as_bytes().len() as _,
            is_bytecode: 0,
        },
        Code::Bytecode(bytes) => c::code_t {
            code: bytes.as_ptr() as _,
            code_len: bytes.len() as _,
            is_bytecode: 1,
        },
    };
    if code.code_len == 0 {
        // Empty String or Vec in Rust would get a invalid ptr of address 0x1
        return Ok(Value::undefined());
    }
    let ret = unsafe { c::js_eval_code(ctx.as_ptr(), &code, &mut callbacks) };
    if ret == 0 {
        userdata.output
    } else {
        let output = userdata.output?;
        if output.is_error() {
            let message = output.to_string();
            let backtrace = output.get_property("stack").unwrap_or_default();
            Err(format!("{}\n{}", message, backtrace))
        } else {
            Err(output.to_string())
        }
    }
}
