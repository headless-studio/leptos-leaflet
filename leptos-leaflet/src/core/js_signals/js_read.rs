use std::panic::Location;

use guards::{Plain, ReadGuard};
use leptos::prelude::*;

use crate::core::thread_safe_jsvalue::ThreadSafeJsValue;

pub struct JsReadSignal<T: 'static> {
    inner: ReadSignal<ThreadSafeJsValue<T>>,
}

impl<T> Clone for JsReadSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for JsReadSignal<T> {}

impl<T> DefinedAt for JsReadSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.inner.defined_at()
    }
}

impl<T: 'static> IsDisposed for JsReadSignal<T> {
    fn is_disposed(&self) -> bool {
        self.inner.is_disposed()
    }
}

impl<T: 'static> ReadUntracked for JsReadSignal<T> {
    type Value = ReadGuard<ThreadSafeJsValue<T>, Plain<ThreadSafeJsValue<T>>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.inner.try_read_untracked()
    }
}

pub struct JsSignal<T: 'static> {
    inner: ReadSignal<ThreadSafeJsValue<T>>,
}

impl<T> Clone for JsSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for JsSignal<T> {}

impl<T: 'static> Dispose for JsSignal<T> {
    fn dispose(self) {
        self.inner.dispose()
    }
}

impl<T> DefinedAt for JsSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.inner.defined_at()
    }
}

impl<T> WithUntracked for JsSignal<T> {
    type Value = T;

    fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        self.inner.try_with_untracked(|v| fun(v))
    }
}

impl<T> With for JsSignal<T>
where
    T: Clone,
{
    type Value = T;

    fn try_with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        self.inner.try_with(|v| fun(v))
    }
}

impl<T> From<JsReadSignal<T>> for ReadSignal<ThreadSafeJsValue<T>> {
    fn from(value: JsReadSignal<T>) -> Self {
        value.inner
    }
}

impl<T> From<JsReadSignal<T>> for JsSignal<T> {
    fn from(value: JsReadSignal<T>) -> Self {
        JsSignal { inner: value.inner }
    }
}

impl<T> From<JsSignal<T>> for JsReadSignal<T> {
    fn from(value: JsSignal<T>) -> Self {
        JsReadSignal { inner: value.inner }
    }
}

impl<T> From<JsSignal<T>> for Signal<ThreadSafeJsValue<T>> {
    fn from(value: JsSignal<T>) -> Self {
        value.inner.into()
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<JsReadSignal<T>> for ReadSignal<ThreadSafeJsValue<T>> {
    fn into(self) -> JsReadSignal<T> {
        JsReadSignal { inner: self }
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<JsSignal<T>> for ReadSignal<ThreadSafeJsValue<T>> {
    fn into(self) -> JsSignal<T> {
        JsSignal { inner: self }
    }
}
