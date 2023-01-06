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
    Source(&'a str),
    Bytecode(&'a [u8]),
}

impl JsCode<'_> {
    fn is_bytecode(&self) -> bool {
        match self {
            JsCode::Source(_) => false,
            JsCode::Bytecode(_) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    String(String),
    Bytes(Vec<u8>),
    Undefined,
}

pub fn eval(script: JsCode, args: &[String]) -> Result<Output, String> {
    let cstr;

    let bytes = match script {
        JsCode::Source(src) => {
            let Ok(script) = alloc::ffi::CString::new(src) else {
                return Err("Invalid script".into());
            };
            cstr = script;
            cstr.as_bytes()
        }
        JsCode::Bytecode(code) => code,
    };

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

    let ret = unsafe {
        c::js_eval(
            bytes.as_ptr() as _,
            bytes.len() as _,
            if script.is_bytecode() { 1 } else { 0 },
            &mut callbacks,
        )
    };
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
