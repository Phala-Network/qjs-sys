use crate::{
    self as js,
    opaque_value::{new_opaque_object, opaque_object_get_data_raw},
};

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use js::{c, Context, FromJsValue, Result, ToJsValue, Value};

pub use gc_mark::{GcMark, NoGc};
mod gc_mark;

pub trait NativeClass: GcMark + 'static {
    const CLASS_NAME: &'static str;
    fn constructor_object(ctx: &Context) -> Result<Value>;
    fn register(ctx: &Context) -> Result<()> {
        Self::constructor_object(ctx)?;
        Ok(())
    }
}

struct Guard<T>(T);

pub struct Ref<'a, T> {
    r: super::opaque_value::Ref<'a, Guard<T>>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self
            .r
            .get()
            .expect("Native object ref should never be None")
            .0
    }
}

pub struct RefMut<'a, T> {
    r: super::opaque_value::RefMut<'a, Guard<T>>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self
            .r
            .get()
            .expect("Native object ref should never be None")
            .0
    }
}

impl<T> DerefMut for RefMut<'_, T> {
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

impl<T: NativeClass> FromJsValue for Native<T> {
    fn from_js_value(value: Value) -> Result<Self> {
        if !value.is_opaque_object_of::<Guard<T>>() {
            return Err(js::Error::Expect(T::CLASS_NAME));
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

impl<T: NativeClass> Native<T> {
    pub fn new(ctx: &Context, value: T) -> Result<Self> {
        let constructor = T::constructor_object(ctx)?;
        let proto = constructor.get_property("prototype")?;
        extern "C" fn gc_mark<T: NativeClass>(
            rt: *mut c::JSRuntime,
            value: c::JSValue,
            mark_fn: c::JS_MarkFunc,
        ) {
            let data = opaque_object_get_data_raw::<Guard<T>>(&value);
            let data = data.get().expect("Native object ref should never be None");
            data.0.gc_mark(rt, mark_fn);
        }
        let object = new_opaque_object(ctx, None, Guard(value), Some(gc_mark::<T>));
        let _ = object.set_prototype(&proto);
        Ok(Self {
            inner: object,
            _marker: PhantomData,
        })
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        Ref {
            r: self.inner.opaque_object_data(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        RefMut {
            r: self.inner.opaque_object_data_mut(),
        }
    }

    pub fn js_value(&self) -> Value {
        self.inner.clone()
    }
}

impl Context {
    pub fn new_native<T: NativeClass>(&self, value: T) -> Result<Native<T>> {
        Native::new(self, value)
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

impl<T: NativeClass> IntoNativeObject for Result<T> {
    type T = T;
    fn into_native_object(self, ctx: &Context) -> Result<Native<Self::T>> {
        self?.into_native_object(ctx)
    }
}
