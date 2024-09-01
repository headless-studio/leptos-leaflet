use std::panic::Location;

use leptos::prelude::*;

use crate::core::thread_safe_jsvalue::ThreadSafeJsValue;

use super::{JsReadSignal, JsSignal, JsWriteSignal};

pub struct JsRwSignal<T> {
    signal: RwSignal<ThreadSafeJsValue<T>>,
}

impl<T> Clone for JsRwSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for JsRwSignal<T> {}

impl<T: Default + 'static> Default for JsRwSignal<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: PartialEq> PartialEq for JsRwSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.signal == other.signal
    }
}

impl<T: Eq> Eq for JsRwSignal<T> {}

impl<T> JsRwSignal<T> {
    pub fn inner(&self) -> &RwSignal<ThreadSafeJsValue<T>> {
        &self.signal
    }
}

impl<T: 'static> JsRwSignal<T> {
    pub fn new(value: T) -> Self {
        Self {
            signal: RwSignal::new(ThreadSafeJsValue::new(value)),
        }
    }

    pub fn read_only(&self) -> JsReadSignal<T> {
        self.signal.read_only().into()
    }

    pub fn write_only(&self) -> JsWriteSignal<T> {
        self.signal.write_only().into()
    }

    pub fn split(&self) -> (JsReadSignal<T>, JsWriteSignal<T>) {
        let (a, b) = self.signal.split();
        (a.into(), b.into())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for JsRwSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsRwSignal")
            .field("signal", &self.signal)
            .finish()
    }
}

impl<T: 'static> JsRwSignal<T> {
    pub fn unite(
        read: ReadSignal<ThreadSafeJsValue<T>>,
        write: WriteSignal<ThreadSafeJsValue<T>>,
    ) -> Option<Self> {
        RwSignal::unite(read, write).map(|signal| Self { signal })
    }

    pub fn set(&self, value: T) {
        self.signal.set(ThreadSafeJsValue::new(value))
    }

    pub fn set_untracked(&self, value: T) {
        self.signal
            .update_untracked(|v| *v = ThreadSafeJsValue::new(value))
    }
}

impl<T> DefinedAt for JsRwSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.signal.defined_at()
    }
}

impl<T: 'static> IsDisposed for JsRwSignal<T> {
    fn is_disposed(&self) -> bool {
        self.signal.is_disposed()
    }
}

impl<T: 'static> WithUntracked for JsRwSignal<T> {
    type Value = T;

    fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        self.signal.try_with_untracked(|v| fun(v))
    }
}

impl<T: 'static> With for JsRwSignal<T> {
    type Value = T;

    fn try_with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        self.signal.try_with(|v| fun(v))
    }
}

impl<T: 'static> Trigger for JsRwSignal<T> {
    fn trigger(&self) {
        self.signal.trigger()
    }
}

impl<T: 'static> Writeable for JsRwSignal<T> {
    type Value = ThreadSafeJsValue<T>;

    fn try_write(&self) -> Option<impl UntrackableGuard<Target = Self::Value>> {
        self.signal.try_write()
    }

    fn try_write_untracked(&self) -> Option<impl std::ops::DerefMut<Target = Self::Value>> {
        self.signal.try_write_untracked()
    }
}

impl<T> From<JsRwSignal<T>> for JsSignal<T> {
    fn from(value: JsRwSignal<T>) -> Self {
        value.read_only().into()
    }
}
