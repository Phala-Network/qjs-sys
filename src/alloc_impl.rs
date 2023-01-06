use super::c;

use core::{
    ffi::c_int,
    mem::{forget, size_of},
};

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

extern "Rust" {
    fn __pink_host_call(
        id: u32,
        ctx: *mut c::JSContext,
        this: c::JSValueConst,
        args: &[c::JSValueConst],
    ) -> c::JSValue;
}

#[no_mangle]
extern "C" fn __host_call(
    ctx: *mut c::JSContext,
    this_val: c::JSValueConst,
    argc: c_int,
    argv: *const c::JSValueConst,
) -> c::JSValue {
    if argc < 1 {
        crate::throw_type_error(ctx, "host call without id");
        return c::JS_EXCEPTION;
    }
    let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
    let mut id = 0;
    if unsafe { c::JS_ToUint32(ctx, &mut id, args[0]) } != 0 {
        crate::throw_type_error(ctx, "invalid host call id");
        return c::JS_EXCEPTION;
    }

    unsafe { __pink_host_call(id, ctx, this_val, &args[1..]) }
}
