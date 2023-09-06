use crate::{c, JsCode, Value};
use core::ptr::NonNull;

pub struct Context {
    pub(crate) ptr: NonNull<c::JSContext>,
}

impl Context {
    pub fn clone_from_ptr(ptr: NonNull<c::JSContext>) -> Self {
        unsafe { c::JS_DupContext(ptr.as_ptr()) };
        Self::new_moved(ptr)
    }

    pub fn new_moved(ptr: NonNull<c::JSContext>) -> Self {
        Context { ptr }
    }

    pub fn ptr(&self) -> NonNull<c::JSContext> {
        self.ptr
    }

    pub fn as_ptr(&self) -> *mut c::JSContext {
        self.ptr.as_ptr()
    }

    pub fn get_global_object(&self) -> Value {
        crate::get_global(self.ptr)
    }

    pub fn new_object(&self) -> Value {
        Value::new_object(self.ptr)
    }

    pub fn new_array(&self) -> Value {
        Value::new_array(self.ptr)
    }

    pub fn eval(&self, code: &JsCode) -> Result<Value, String> {
        crate::eval(self.ptr, code)
    }
}

impl AsRef<c::JSContext> for Context {
    fn as_ref(&self) -> &c::JSContext {
        unsafe { self.ptr.as_ref() }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        let ptr = unsafe { c::JS_DupContext(self.ptr.as_ptr()) };
        let ptr = NonNull::new(ptr).expect("Failed to clone JSContext");
        Context { ptr }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            c::JS_FreeContext(self.ptr.as_ptr());
        }
    }
}

pub struct Runtime {
    ptr: NonNull<c::JSRuntime>,
}

impl Runtime {
    pub fn new() -> Self {
        let ptr = unsafe { c::JS_NewRuntime() };
        let ptr = NonNull::new(ptr).expect("Failed to create JSRuntime");
        Runtime { ptr }
    }

    pub fn new_context(&self) -> Context {
        let ptr = unsafe { c::JS_NewContext(self.ptr.as_ptr()) };
        let ptr = NonNull::new(ptr).expect("Failed to create JSContext");
        Context { ptr }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            c::JS_FreeRuntime(self.ptr.as_ptr());
        }
    }
}
