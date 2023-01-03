#![no_std]
extern crate alloc;

use core::ffi::CStr;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

mod alloc_impl {
    use core::mem::{forget, size_of};

    use alloc::vec::Vec;

    #[no_mangle]
    extern "C" fn __pink_malloc(size: usize) -> *mut ::core::ffi::c_void {
        let mut buf = Vec::<usize>::new();
        if buf.try_reserve(cap(size)).is_err() {
            return core::ptr::null_mut();
        }
        buf.push(buf.capacity());
        let ptr = unsafe { buf.as_mut_ptr().offset(1) as *mut ::core::ffi::c_void };
        forget(buf);
        ptr
    }

    #[no_mangle]
    extern "C" fn __pink_free(ptr: *mut ::core::ffi::c_void) {
        drop(recover(ptr));
    }

    #[no_mangle]
    extern "C" fn __pink_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: usize,
    ) -> *mut ::core::ffi::c_void {
        match recover(ptr) {
            Some(mut buffer) => {
                let cap_required = cap(size);
                if cap_required > buffer.capacity() {
                    if buffer
                        .try_reserve(cap_required - buffer.capacity())
                        .is_err()
                    {
                        return core::ptr::null_mut();
                    }
                    buffer[0] = buffer.capacity();
                }
                let ptr = unsafe { buffer.as_mut_ptr().offset(1) as *mut ::core::ffi::c_void };
                forget(buffer);
                ptr
            }
            None => __pink_malloc(size),
        }
    }

    fn cap(size: usize) -> usize {
        size / size_of::<usize>() + 2
    }

    fn recover(ptr: *mut ::core::ffi::c_void) -> Option<Vec<usize>> {
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let ptr = (ptr as *mut usize).offset(-1);
            let capacity = *ptr;
            let buf = Vec::<usize>::from_raw_parts(ptr, capacity, capacity);
            Some(buf)
        }
    }
}

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
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
        handler: sys::input_handler_t,
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
        let bytes: &[u8] = unsafe { &*core::ptr::slice_from_raw_parts(output as _, output_len as _) };
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

    let mut callbacks = sys::callbacks_t {
        userdata: &mut userdata as *mut _ as *mut ::core::ffi::c_void,
        output_str: Some(output_str),
        output_bytes: Some(output_bytes),
        read_args: Some(read_args),
    };

    let ret = unsafe {
        sys::js_eval(
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
