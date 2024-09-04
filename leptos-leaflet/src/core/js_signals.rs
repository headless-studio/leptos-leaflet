use leptos::prelude::*;

pub type JsRwSignal<T> = RwSignal<T, LocalStorage>;
pub type JsSignal<T> = Signal<T, LocalStorage>;
pub type JsReadSignal<T> = ReadSignal<T, LocalStorage>;
pub type JsWriteSignal<T> = WriteSignal<T, LocalStorage>;
pub type JsStoredValue<T> = StoredValue<T, LocalStorage>;
pub type JsMaybeSignal<T> = MaybeSignal<T, LocalStorage>;