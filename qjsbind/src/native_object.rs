use crate as js;

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use js::{Context, FromJsValue, Result, ToJsValue, Value};

pub trait NativeClass: 'static {
    const CLASS_NAME: &'static str;
    fn class_object(ctx: &Context) -> Value;
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

impl<T: 'static> FromJsValue for Native<T> {
    fn from_js_value(value: Value) -> Result<Self> {
        if !value.is_opaque_object_of::<Guard<T>>() {
            return Err(js::Error::Custom(format!(
                "expected {}",
                crate::type_name::<T>()
            )));
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
    pub fn new(ctx: &Context, value: T) -> Self {
        let class_obj = T::class_object(ctx);
        let object = Value::new_opaque_object(ctx, Guard(value));
        let _ = object.set_prototype(&class_obj);
        Self {
            inner: object,
            _marker: PhantomData,
        }
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
}

impl Context {
    pub fn new_native<T: NativeClass>(&self, value: T) -> Native<T> {
        Native::new(self, value)
    }
}
