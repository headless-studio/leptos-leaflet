use std::panic::Location;

use leptos::prelude::*;

use crate::core::thread_safe_jsvalue::ThreadSafeJsValue;

pub struct JsStoredValue<T> {
    value: StoredValue<ThreadSafeJsValue<T>>,
}

impl<T> Clone for JsStoredValue<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for JsStoredValue<T> {}

impl<T: 'static> JsStoredValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: StoredValue::new(ThreadSafeJsValue::new(value)),
        }
    }
}

impl<T: Default + 'static> Default for JsStoredValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: PartialEq> PartialEq for JsStoredValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for JsStoredValue<T> {}

impl<T> JsStoredValue<T> {
    pub fn inner(&self) -> &StoredValue<ThreadSafeJsValue<T>> {
        &self.value
    }
}

impl<T: 'static> JsStoredValue<T> {
    pub fn set_value(&self, value: T) {
        self.value.set_value(ThreadSafeJsValue::new(value));
    }

    pub fn try_set_value(&self, value: T) -> Option<ThreadSafeJsValue<T>> {
        self.value.try_set_value(ThreadSafeJsValue::new(value))
    }

    pub fn with_value<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        self.value.with_value(|v| f(v))
    }

    pub fn update_value(&self, f: impl FnOnce(&mut T)) {
        self.value.update_value(|v| f(&mut *v))
    }
}

impl<T> JsStoredValue<T>
where
    T: Clone + 'static,
{
    pub fn get_value(&self) -> ThreadSafeJsValue<T> {
        self.value.get_value().clone()
    }

    pub fn try_with_value<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        self.value.try_with_value(|v| f(v))
    }

    pub fn try_get_value(&self) -> Option<ThreadSafeJsValue<T>> {
        self.value.try_get_value().clone()
    }
}

impl<T> DefinedAt for JsStoredValue<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.value.defined_at()
    }
}

impl<T> Dispose for JsStoredValue<T> {
    fn dispose(self) {
        self.value.dispose();
    }
}