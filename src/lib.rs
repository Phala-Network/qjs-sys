#![no_std]
extern crate alloc;

use alloc::ffi::CString;

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
pub fn eval(script: &str) -> i32 {
    let Ok(script) = CString::new(script) else {
        return -1;
    };
    let bytes = script.as_bytes();
    unsafe { sys::js_eval_oneshot(bytes.as_ptr() as _, bytes.len() as _) }
}
