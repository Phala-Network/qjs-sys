use core::ptr::NonNull;
use std::time::Instant;

use crate::{c, Code, Result, ToJsValue, Value};
use alloc::string::{String, ToString};
use anyhow::{anyhow, bail, Context as _};
use qjs_sys::inline_fns::JSCFunction;
use tokio::sync::broadcast;

pub struct PauseGc {
    ctx: Context,
}

impl PauseGc {
    pub fn new(ctx: Context) -> Self {
        unsafe {
            let rt = c::JS_GetRuntime(ctx.as_ptr());
            c::JS_PauseGC(rt);
        }
        PauseGc { ctx }
    }
}

impl Drop for PauseGc {
    fn drop(&mut self) {
        unsafe {
            let rt = c::JS_GetRuntime(self.ctx.as_ptr());
            c::JS_ResumeGC(rt);
        }
    }
}

pub struct Context {
    pub(crate) ptr: NonNull<c::JSContext>,
}

impl Context {
    pub fn clone_from_ptr(ptr: *mut c::JSContext) -> Option<Self> {
        let ptr = NonNull::new(ptr)?;
        unsafe { c::JS_DupContext(ptr.as_ptr()) };
        Some(Self { ptr })
    }

    pub fn as_ptr(&self) -> *mut c::JSContext {
        self.ptr.as_ptr()
    }

    pub fn get_global_object(&self) -> Value {
        crate::get_global(self)
    }

    pub fn new_object(&self, name: &str) -> Value {
        Value::new_object(self, name)
    }

    pub fn new_array(&self) -> Value {
        Value::new_array(self)
    }

    pub fn new_string(&self, s: &str) -> Value {
        Value::from_str(self, s)
    }

    pub fn eval(&self, code: &Code) -> Result<Value, String> {
        crate::eval(self, code)
    }

    pub fn throw(&self, err: impl core::fmt::Display) {
        self.throw_str(&format!("{err:#}"));
    }

    pub fn throw_dbg(&self, err: impl core::fmt::Debug) {
        self.throw_str(&format!("{err:#?}"));
    }

    pub fn throw_str(&self, err: &str) {
        let cmsg = alloc::ffi::CString::new(err).unwrap_or_default();
        unsafe {
            c::JS_ThrowGenericError(self.as_ptr(), cmsg.as_ptr());
        }
    }

    pub fn throw_type_err(&self, err: &str) {
        let cmsg = alloc::ffi::CString::new(err).unwrap_or_default();
        unsafe { c::JS_ThrowTypeError(self.as_ptr(), cmsg.as_ptr()) };
    }

    pub fn get_exception_error(&self) -> crate::Error {
        anyhow!("{}", self.get_exception_str())
    }

    pub fn get_exception_str(&self) -> String {
        let ctx_ptr = self.as_ptr();
        unsafe {
            let e = c::JS_GetException(ctx_ptr);
            let mut exc_str = crate::ctx_to_string(self, e);
            let stack = c::JS_GetPropertyStr(ctx_ptr, e, cstr::cstr!("stack").as_ptr() as _);
            if !c::is_undefined(stack) {
                exc_str.push_str("\n[stack]\n");
                exc_str.push_str(&crate::ctx_to_string(self, stack));
            }
            c::JS_FreeValue(ctx_ptr, e);
            c::JS_FreeValue(ctx_ptr, stack);
            exc_str
        }
    }

    pub fn get_qjsbind_object<F, V>(&self, name: &str, or_default: F) -> Result<Value>
    where
        F: Fn() -> Result<V>,
        V: ToJsValue,
    {
        let global = self.get_global_object();
        let bindings_obj_name = "_QjsBind";
        let mut bindings = global
            .get_property(bindings_obj_name)
            .context("failed to get global _QjsBind object")?;
        if bindings.is_undefined() {
            bindings = self.new_object(bindings_obj_name);
            global.set_property(bindings_obj_name, &bindings)?;
        }
        let mut obj = bindings.get_property(name)?;
        if obj.is_undefined() {
            obj = or_default()?.to_js_value(self)?;
            bindings.set_property(name, &obj)?;
        }
        Ok(obj)
    }

    pub fn resolve_object(&self, full_path: &str) -> Result<Value> {
        let mut result = self.get_global_object();
        for seg in full_path.split('.') {
            if result.is_undefined() {
                bail!("lookup_object: {full_path} is undefined");
            }
            result = result.get_property(seg)?;
        }
        Ok(result)
    }

    pub fn new_function(
        &self,
        name: &str,
        func: JSCFunction,
        argc: u32,
        ty: c::JSCFunctionEnum,
    ) -> Value {
        let f = unsafe {
            c::JS_NewCFunction2Len(
                self.as_ptr(),
                Some(func),
                name.as_ptr() as _,
                name.len() as _,
                argc as _,
                ty,
                0,
            )
        };
        Value::new_moved(self, f)
    }

    pub fn pause_gc(&self) -> PauseGc {
        PauseGc::new(self.clone())
    }
}

impl AsRef<c::JSContext> for Context {
    fn as_ref(&self) -> &c::JSContext {
        unsafe { self.ptr.as_ref() }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        unsafe { c::JS_DupContext(self.ptr.as_ptr()) };
        Context { ptr: self.ptr }
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

#[derive(Debug, Clone, Default)]
pub struct EngineConfig {
    pub memory_limit: Option<u32>,
    pub gas_limit: Option<u32>,
    pub time_limit: Option<u64>,
}

impl EngineConfig {
    pub fn need_interrupt(&self) -> bool {
        self.gas_limit.is_some() || self.time_limit.is_some()
    }
}

struct RuntimeData {
    gas_remain: u32,
    abort_tx: Option<broadcast::Sender<()>>,
    start_time: Instant,
    time_limit: Option<u64>,
}

extern "C" fn interrupt_handler(rt: *mut c::JSRuntime, _opaque: *mut core::ffi::c_void) -> i32 {
    let data = unsafe { &mut *(c::JS_GetRuntimeOpaque(rt) as *mut RuntimeData) };
    if data.gas_remain == 0 {
        if let Some(tx) = &data.abort_tx {
            let _ = tx.send(());
        }
        return 1;
    }
    if let Some(time_limit) = data.time_limit {
        let elapsed = data.start_time.elapsed();
        if elapsed.as_millis() >= time_limit as _ {
            if let Some(tx) = &data.abort_tx {
                let _ = tx.send(());
            }
            return 1;
        }
    }
    data.gas_remain -= 1;
    0
}

impl Runtime {
    pub fn new(config: &EngineConfig) -> Self {
        let ptr = unsafe { c::JS_NewRuntime() };
        let ptr = NonNull::new(ptr).expect("Failed to create JSRuntime");

        let gas_remain = config.gas_limit.unwrap_or_default();
        let data = Box::new(RuntimeData {
            gas_remain,
            start_time: Instant::now(),
            time_limit: config.time_limit,
            abort_tx: None,
        });
        unsafe {
            c::JS_SetRuntimeOpaque(ptr.as_ptr(), Box::into_raw(data) as *mut _);

            if config.need_interrupt() {
                c::JS_SetInterruptHandler(
                    ptr.as_ptr(),
                    Some(interrupt_handler),
                    core::ptr::null_mut(),
                );
            }
            if let Some(memory_limit) = config.memory_limit {
                c::JS_SetMemoryLimit(ptr.as_ptr(), memory_limit as usize);
            }
        }
        Runtime { ptr }
    }

    pub fn new_context(&self) -> Context {
        let ptr = unsafe { c::JS_NewContext(self.ptr.as_ptr()) };
        let ptr = NonNull::new(ptr).expect("Failed to create JSContext");
        unsafe {
            c::js_opaque_class_init(ptr.as_ptr());
        }
        Context { ptr }
    }

    pub fn exec_pending_jobs(&self) -> Result<i32, String> {
        let mut ctx_ptr = core::ptr::null_mut();
        let ret = unsafe { c::JS_ExecutePendingJob(self.ptr.as_ptr(), &mut ctx_ptr) };
        if ret < 0 {
            return match Context::clone_from_ptr(ctx_ptr) {
                Some(ctx) => Err(ctx.get_exception_str()),
                None => Err("no context".to_string()),
            };
        }
        Ok(ret)
    }

    pub fn enable_dump_exceptions(&self) {
        unsafe {
            let flags = c::JS_GetDebugFlags(self.ptr.as_ptr());
            c::JS_SetDebugFlags(self.ptr.as_ptr(), flags | c::JS_DF_DUMP_EXCEPTIONS);
        }
    }

    pub fn enable_dump_undefine_properrties(&self) {
        unsafe {
            let flags = c::JS_GetDebugFlags(self.ptr.as_ptr());
            c::JS_SetDebugFlags(self.ptr.as_ptr(), flags | c::JS_DF_DUMP_UNDEFINED_PROPS);
        }
    }

    pub fn set_debug_flags(&self, flags: u32) {
        unsafe {
            c::JS_SetDebugFlags(self.ptr.as_ptr(), flags);
        }
    }

    pub fn subscribe_abort(&self) -> broadcast::Receiver<()> {
        let data = unsafe { &mut *(c::JS_GetRuntimeOpaque(self.ptr.as_ptr()) as *mut RuntimeData) };
        if let Some(tx) = &data.abort_tx {
            return tx.subscribe();
        }
        let (tx, rx) = broadcast::channel(1);
        data.abort_tx = Some(tx);
        rx
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            let data = c::JS_GetRuntimeOpaque(self.ptr.as_ptr());
            let data = Box::from_raw(data as *mut RuntimeData);
            drop(data);
            c::JS_FreeRuntime(self.ptr.as_ptr());
        }
    }
}
