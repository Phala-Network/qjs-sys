use core::{any::TypeId, ptr::NonNull};

use alloc::boxed::Box;
use qjs_sys::c;

use crate::Value;

fn type_id<T: 'static>() -> u64 {
    let type_id = TypeId::of::<T>();
    let mut tag: u64 = 0;
    // read 64bits of the type id as tag
    unsafe {
        core::ptr::copy_nonoverlapping(
            &type_id as *const _ as *const u8,
            &mut tag as *mut _ as *mut u8,
            core::mem::size_of::<u64>().min(core::mem::size_of::<TypeId>()),
        );
    };
    tag
}

pub fn new_opaque_object<T: 'static>(ctx: NonNull<c::JSContext>, value: T) -> Value {
    extern "C" fn free_opaque<T>(
        _rt: *mut c::JSRuntime,
        data: *mut ::core::ffi::c_void,
        _tag: ::core::ffi::c_int,
    ) {
        let _drop_it = unsafe { Box::from_raw(data as *mut T) };
    }
    let boxed = Box::new(value);
    let data = Box::into_raw(boxed);
    let tag: u64 = type_id::<T>();
    let js_value = unsafe {
        c::JS_OpaqueObjectNew(
            ctx.as_ptr(),
            data as *mut _,
            Some(free_opaque::<T>),
            tag as _,
        )
    };
    Value::new_moved(ctx, js_value)
}

pub fn opaque_object_get_data<T: 'static>(value: &Value) -> Option<&T> {
    let Value::Other { value, ctx } = value  else {
        return None;
    };
    let ptr = unsafe { c::JS_OpaqueObjectDataGet(ctx.as_ptr(), *value, type_id::<T>() as _) };
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { &*(ptr as *const T) })
}

pub fn opaque_object_take_data<T: 'static>(value: &Value) -> Option<Box<T>> {
    let Value::Other { value, ctx } = value  else {
        return None;
    };
    unsafe {
        let ptr = c::JS_OpaqueObjectDataGet(ctx.as_ptr(), *value, type_id::<T>() as _);
        if ptr.is_null() {
            return None;
        }
        c::JS_OpaqueObjectDataForget(ctx.as_ptr(), *value);
        Some(Box::from_raw(ptr as *mut T))
    }
}
