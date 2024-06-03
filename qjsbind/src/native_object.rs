use crate::{
    self as js,
    error::expect_js_value,
    opaque_value::{new_opaque_object, opaque_object_get_data_raw},
};

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use js::{c, Context, FromJsValue, Result, ToJsValue, Value};

pub use gc_mark::{GcMark, NoGc};
mod gc_mark;

pub trait Named {
    const CLASS_NAME: &'static str;
}

#[macro_export(local_inner_macros)]
macro_rules! impl_named {
    ($t:ty as $name:expr) => {
        impl $crate::Named for $t {
            const CLASS_NAME: &'static str = $name;
        }
    };
    ($t:ty) => {
        impl_named!($t, stringify!($t));
    };
}

pub trait NativeClass: GcMark + Named + 'static {
    fn constructor_object(ctx: &Context) -> Result<Value>;
    fn register(ctx: &Context) -> Result<()> {
        Self::constructor_object(ctx)?;
        Ok(())
    }
}

struct Guard<T>(T);

pub struct NativeValueRef<'a, T> {
    r: super::opaque_value::Ref<'a, Guard<T>>,
}

impl<T> Deref for NativeValueRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self
            .r
            .get()
            .expect("Native object ref should never be None")
            .0
    }
}

pub struct NativeValueRefMut<'a, T> {
    r: super::opaque_value::RefMut<'a, Guard<T>>,
}

impl<T> Deref for NativeValueRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self
            .r
            .get()
            .expect("Native object ref should never be None")
            .0
    }
}

impl<T> DerefMut for NativeValueRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self
            .r
            .get_mut()
            .expect("Native object ref should never be None")
            .0
    }
}

pub struct Native<T> {
    inner: Value,
    _marker: PhantomData<T>,
}

impl<T> Clone for Native<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: GcMark + Named + 'static> FromJsValue for Native<T> {
    fn from_js_value(value: Value) -> Result<Self> {
        if !value.is_opaque_object_of::<Guard<T>>() {
            return Err(expect_js_value(&value, T::CLASS_NAME));
        }
        Ok(Self {
            inner: value,
            _marker: PhantomData,
        })
    }
}

impl<T: 'static> ToJsValue for Native<T> {
    fn to_js_value(&self, _ctx: &crate::Context) -> Result<Value> {
        Ok(self.inner.clone())
    }
}

impl<T> From<Native<T>> for Value {
    fn from(obj: Native<T>) -> Self {
        obj.inner
    }
}

impl<T: 'static> Native<T> {
    pub fn borrow(&self) -> NativeValueRef<'_, T> {
        NativeValueRef {
            r: self.inner.opaque_object_data(),
        }
    }

    pub fn borrow_mut(&self) -> NativeValueRefMut<'_, T> {
        NativeValueRefMut {
            r: self.inner.opaque_object_data_mut(),
        }
    }

    pub fn js_value(&self) -> Value {
        self.inner.clone()
    }
}

impl<T: GcMark + Named + 'static> Native<T> {
    pub fn new_gc_obj_named(ctx: &Context, opaque_value: T) -> Result<Self> {
        extern "C" fn gc_mark<T: GcMark + 'static>(
            rt: *mut c::JSRuntime,
            value: c::JSValue,
            mark_fn: c::JS_MarkFunc,
        ) {
            let data = opaque_object_get_data_raw::<Guard<T>>(&value);
            let data = data.get().expect("Native object ref should never be None");
            data.0.gc_mark(rt, mark_fn);
        }
        let object = new_opaque_object(
            ctx,
            Some(T::CLASS_NAME),
            Guard(opaque_value),
            Some(gc_mark::<T>),
        );
        Ok(Self {
            inner: object,
            _marker: PhantomData,
        })
    }
}

impl<T: NativeClass> Native<T> {
    pub fn new(ctx: &Context, value: T) -> Result<Self> {
        let constructor = T::constructor_object(ctx)?;
        let proto = constructor.get_property("prototype")?;
        let object = Self::new_gc_obj_named(ctx, value)?;
        _ = object.inner.set_prototype(&proto);
        Ok(object)
    }
}

impl Context {
    pub fn wrap_native<T: NativeClass>(&self, value: T) -> Result<Native<T>> {
        Native::new(self, value)
    }

    pub fn new_gc_opaque_named<T: GcMark + Named + 'static>(&self, value: T) -> Result<Native<T>> {
        Native::new_gc_obj_named(self, value)
    }
}

pub trait IntoNativeObject {
    type T: NativeClass;
    fn into_native_object(self, ctx: &Context) -> Result<Native<Self::T>>;
}

impl<T: NativeClass> IntoNativeObject for T {
    type T = T;
    fn into_native_object(self, ctx: &Context) -> Result<Native<Self::T>> {
        Native::new(ctx, self)
    }
}

impl<T, E> IntoNativeObject for Result<T, E>
where
    T: NativeClass,
    crate::Error: From<E>,
{
    type T = T;
    fn into_native_object(self, ctx: &Context) -> Result<Native<Self::T>> {
        Ok(self?.into_native_object(ctx)?)
    }
}
