use crate::{c, Native, Value};

pub trait GcMark {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc);
}

macro_rules! impl_gc_mark_for {
    ($($t:ty),*) => {
        $(
            impl GcMark for $t {
                fn gc_mark(&self, _rt: *mut c::JSRuntime, _mark_fn: c::JS_MarkFunc) {}
            }
        )*
    };
}

impl_gc_mark_for! {
    i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char,
    alloc::string::String, alloc::ffi::CString
}

#[cfg(feature = "std")]
impl_gc_mark_for! {
    std::ffi::OsString, std::ffi::OsStr, std::path::PathBuf, std::path::Path,
    std::time::Duration, std::time::Instant
}

impl<T> GcMark for Native<T> {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        self.inner.gc_mark(rt, mark_fn)
    }
}

impl<T: GcMark> GcMark for Option<T> {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        if let Some(value) = self {
            value.gc_mark(rt, mark_fn);
        }
    }
}

impl<T> GcMark for alloc::vec::Vec<T>
where
    T: GcMark,
{
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        for value in self {
            value.gc_mark(rt, mark_fn);
        }
    }
}

impl GcMark for Value {
    fn gc_mark(&self, rt: *mut c::JSRuntime, mark_fn: c::JS_MarkFunc) {
        let Ok(ctx) = self.context() else {
            return;
        };
        unsafe {
            c::JS_MarkValue(rt, *self.raw_value(), mark_fn);
            c::JS_MarkContext(rt, ctx.as_ptr(), mark_fn);
        }
    }
}

/// A wrapper type that does not participate in JS garbage collection.
pub struct NoGc<T>(pub T);
impl<T> GcMark for NoGc<T> {
    fn gc_mark(&self, _rt: *mut c::JSRuntime, _mark_fn: c::JS_MarkFunc) {}
}
impl<T> NoGc<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}
impl<T> From<T> for NoGc<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T> core::ops::Deref for NoGc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for NoGc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
