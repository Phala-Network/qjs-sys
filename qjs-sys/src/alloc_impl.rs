#[cfg(feature = "pink-allocator")]
mod pink_allocator {
    use alloc::vec::Vec;
    use core::mem::{forget, size_of};

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
                        .try_reserve(cap_required.saturating_sub(buffer.capacity()))
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

    #[allow(clippy::arithmetic_side_effects)]
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

    #[no_mangle]
    extern "C" fn __pink_malloc_usable_size(ptr: *mut ::core::ffi::c_void) -> usize {
        alloced_size(ptr).unwrap_or(0)
    }

    fn alloced_size(ptr: *mut ::core::ffi::c_void) -> Option<usize> {
        let recovered = recover(ptr)?;
        let size = (recovered.capacity() - 1)
            .checked_mul(size_of::<usize>())
            .expect("invalid alloced size");
        forget(recovered);
        Some(size)
    }
}
#[cfg(feature = "classic-host-call")]
extern "Rust" {
    fn __pink_host_call(id: u32, ctx: *mut c::JSContext, args: &[c::JSValueConst]) -> c::JSValue;
}

#[cfg(feature = "classic-host-call")]
use {super::c, alloc::ffi::CString};
#[cfg(feature = "classic-host-call")]
#[no_mangle]
extern "C" fn __host_call(
    ctx: *mut c::JSContext,
    _this_val: c::JSValueConst,
    argc: core::ffi::c_int,
    argv: *const c::JSValueConst,
) -> c::JSValue {
    pub fn throw(ctx: *mut c::JSContext, msg: &str) -> c::JSValue {
        let cmsg = CString::new(msg).unwrap_or_default();
        unsafe { c::JS_ThrowTypeError(ctx, cmsg.as_ptr()) }
    }

    if argc < 1 {
        throw(ctx, "host call without id");
        return c::JS_EXCEPTION;
    }
    let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
    let mut id = 0;
    if unsafe { c::JS_ToUint32(ctx, &mut id, args[0]) } != 0 {
        throw(ctx, "invalid host call id");
        return c::JS_EXCEPTION;
    }

    unsafe { __pink_host_call(id, ctx, &args[1..]) }
}

#[cfg(feature = "with-polyfills")]
mod polyfills {
    use core::ffi::{c_int, c_uchar};

    #[no_mangle]
    extern "C" fn __pink_getrandom(_pbuf: *mut u8, _nbytes: u8) {}

    #[cfg(feature = "classic-host-call")]
    use super::c;
    #[cfg(feature = "classic-host-call")]
    #[no_mangle]
    fn __pink_host_call(
        _id: u32,
        _ctx: *mut c::JSContext,
        _args: &[c::JSValueConst],
    ) -> super::c::JSValue {
        super::c::JS_EXCEPTION
    }
    #[no_mangle]
    extern "C" fn __pink_clock_time_get(_id: u32, _precision: u64, _retptr0: *mut u64) -> u16 {
        0
    }

    #[no_mangle]
    extern "C" fn __pink_fd_write(_fd: c_int, _buf: *const c_uchar, _len: usize) -> usize {
        unimplemented!()
    }
}
