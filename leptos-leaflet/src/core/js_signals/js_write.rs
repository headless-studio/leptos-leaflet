use std::panic::Location;

use leptos::prelude::*;

use crate::core::thread_safe_jsvalue::ThreadSafeJsValue;

#[derive(Copy, Clone)]
pub struct JsWriteSignal<T> {
    signal: WriteSignal<ThreadSafeJsValue<T>>,
}

impl<T> DefinedAt for JsWriteSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.signal.defined_at()
    }
}

impl<T: 'static> IsDisposed for JsWriteSignal<T> {
    fn is_disposed(&self) -> bool {
        self.signal.is_disposed()
    }
}

impl<T: 'static> Trigger for JsWriteSignal<T> {
    fn trigger(&self) {
        self.signal.trigger()
    }
}

impl<T: 'static> Writeable for JsWriteSignal<T> {
    type Value = ThreadSafeJsValue<T>;

    fn try_write(&self) -> Option<impl UntrackableGuard<Target = Self::Value>> {
        self.signal.try_write()
    }

    fn try_write_untracked(&self) -> Option<impl std::ops::DerefMut<Target = Self::Value>> {
        self.signal.try_write_untracked()
    }
}

impl<T> From<JsWriteSignal<T>> for WriteSignal<ThreadSafeJsValue<T>> {
    fn from(value: JsWriteSignal<T>) -> Self {
        value.signal
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<JsWriteSignal<T>> for WriteSignal<ThreadSafeJsValue<T>> {
    fn into(self) -> JsWriteSignal<T> {
        JsWriteSignal { signal: self }
    }
}