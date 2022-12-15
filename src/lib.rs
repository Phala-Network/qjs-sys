#![no_std]
extern crate alloc;

use core::ffi::CStr;

use alloc::string::String;

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

#[no_mangle]
pub fn eval(script: &str) -> Result<String, String> {
    let Ok(script) = alloc::ffi::CString::new(script) else {
        return Err("Invalid script".into());
    };
    let bytes = script.as_bytes();

    let mut output = String::new();

    extern "C" fn callback(buf_ptr: *mut core::ffi::c_void, output: *const core::ffi::c_char) {
        let buf = unsafe { &mut *(buf_ptr as *mut String) };
        let cstr = unsafe { CStr::from_ptr(output) };
        let s = cstr.to_str().unwrap_or("<Invalid UTF8 sequnece>");
        buf.push_str(s);
    }

    let ret = unsafe {
        sys::js_evaluate(
            bytes.as_ptr() as _,
            bytes.len() as _,
            &mut output as *mut String as _,
            Some(callback),
        )
    };
    if ret == 0 {
        Ok(output)
    } else {
        Err(output)
    }
}
