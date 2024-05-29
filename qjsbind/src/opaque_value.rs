use core::any::TypeId;
use core::cell::RefCell;

use alloc::boxed::Box;
use log::debug;
use qjs_sys::c;

use crate as js;
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

struct Cell<T> {
    cell: RefCell<Option<T>>,
}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Self {
            cell: RefCell::new(Some(value)),
        }
    }

    fn take(&self) -> Option<T> {
        self.cell.borrow_mut().take()
    }
}

#[derive(Default)]
pub struct Ref<'a, T> {
    cell: Option<core::cell::Ref<'a, Option<T>>>,
}

impl<T> Ref<'_, T> {
    fn none() -> Self {
        Self { cell: None }
    }
    pub fn get(&self) -> Option<&T> {
        self.cell.as_ref()?.as_ref()
    }
}

#[derive(Default)]
pub struct RefMut<'a, T> {
    cell: Option<core::cell::RefMut<'a, Option<T>>>,
}

impl<T> RefMut<'_, T> {
    fn none() -> Self {
        Self { cell: None }
    }

    pub fn get(&self) -> Option<&T> {
        self.cell.as_ref()?.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.cell.as_mut()?.as_mut()
    }
}

pub fn new_opaque_object<T: 'static>(
    ctx: &js::Context,
    value: T,
    gc_mark: c::JSClassGCMark,
) -> Value {
    extern "C" fn free_opaque<T>(
        _rt: *mut c::JSRuntime,
        data: *mut ::core::ffi::c_void,
        _tag: ::core::ffi::c_int,
    ) {
        let _drop_it = unsafe { Box::from_raw(data as *mut Cell<T>) };
    }
    debug!(
        "new_opaque_object TID={}, T={:?}",
        type_id::<T>(),
        core::any::type_name::<T>()
    );
    let boxed = Box::new(Cell::new(value));
    let data = Box::into_raw(boxed);
    let tag: u64 = type_id::<T>();
    let js_value = unsafe {
        c::JS_OpaqueObjectNew(
            ctx.as_ptr(),
            data as *mut _,
            Some(free_opaque::<T>),
            gc_mark,
            tag as _,
        )
    };
    Value::new_moved(ctx, js_value)
}

pub fn is_opaque_object_of<T: 'static>(value: &Value) -> bool {
    let Value::Other { value, ctx: _ } = value else {
        return false;
    };
    let ptr = unsafe { c::JS_OpaqueObjectDataGet(*value, type_id::<T>() as _) };
    !ptr.is_null()
}

pub fn opaque_object_get_data<T: 'static>(value: &Value) -> Ref<'_, T> {
    debug!(
        "opaque_object_get_data TID={}, T={:?}",
        type_id::<T>(),
        core::any::type_name::<T>()
    );
    let Value::Other { value, ctx: _ } = value else {
        return Ref::none();
    };
    opaque_object_get_data_raw(value)
}

pub fn opaque_object_get_data_raw<T: 'static>(value: &c::JSValue) -> Ref<'_, T> {
    let ptr = unsafe { c::JS_OpaqueObjectDataGet(*value, type_id::<T>() as _) };
    if ptr.is_null() {
        return Ref::none();
    }
    let cell = unsafe { &*(ptr as *const Cell<T>) };
    Ref {
        cell: Some(cell.cell.borrow()),
    }
}

pub fn opaque_object_get_data_mut<T: 'static>(value: &Value) -> RefMut<'_, T> {
    debug!(
        "opaque_object_get_data_mut TID={}, T={:?}",
        type_id::<T>(),
        core::any::type_name::<T>()
    );
    let Value::Other { value, ctx: _ } = value else {
        return RefMut::none();
    };
    let ptr = unsafe { c::JS_OpaqueObjectDataGet(*value, type_id::<T>() as _) };
    if ptr.is_null() {
        return RefMut::none();
    }
    let cell = unsafe { &mut *(ptr as *mut Cell<T>) };
    RefMut {
        cell: Some(cell.cell.borrow_mut()),
    }
}

pub fn opaque_object_take_data<T: 'static>(value: &Value) -> Option<T> {
    debug!(
        "opaque_object_take_data TID={}, T={:?}",
        type_id::<T>(),
        core::any::type_name::<T>()
    );
    let Value::Other { value, ctx: _ } = value else {
        return None;
    };
    unsafe {
        let ptr = c::JS_OpaqueObjectDataGet(*value, type_id::<T>() as _);
        if ptr.is_null() {
            return None;
        }
        let cell = &*(ptr as *const Cell<T>);
        cell.take()
    }
}
