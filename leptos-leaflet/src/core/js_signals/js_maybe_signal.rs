use std::panic::Location;

use leptos::prelude::*;

use super::{JsReadSignal, JsSignal, JsStoredValue};

pub enum JsMaybeSignal<T>
where
    T: 'static,
{
    /// An unchanging value of type `T`.
    Static(JsStoredValue<T>),
    /// A reactive signal that contains a value of type `T`.
    Dynamic(JsSignal<T>),
}

impl<T> Clone for JsMaybeSignal<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(value) => Self::Static(*value),
            Self::Dynamic(signal) => Self::Dynamic(*signal),
        }
    }
}

impl<T> Copy for JsMaybeSignal<T> where T: Copy {}

impl<T> Default for JsMaybeSignal<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Static(JsStoredValue::new(T::default()))
    }
}

impl<T> JsMaybeSignal<T> where T: 'static {
    pub fn new(value: T) -> Self {
        Self::Static(JsStoredValue::new(value))
    }
}

impl<T: 'static> Dispose for JsMaybeSignal<T> {
    fn dispose(self) {
        match self {
            Self::Static(value) => value.dispose(),
            Self::Dynamic(signal) => signal.dispose(),
        }
    }
}

impl<T> DefinedAt for JsMaybeSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        match self {
            Self::Static(value) => value.defined_at(),
            Self::Dynamic(signal) => signal.defined_at(),
        }
    }
}

impl<T> WithUntracked for JsMaybeSignal<T>
where
    T: 'static,
{
    type Value = T;

    fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        match self {
            Self::Static(value) => value.try_with_untracked(|v| fun(v)),
            Self::Dynamic(signal) => signal.try_with_untracked(|v| fun(v)),
        }
    }
}

impl<T> With for JsMaybeSignal<T>
where
    T: Clone,
{
    type Value = T;

    fn try_with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        match self {
            Self::Static(value) => value.try_with_value(|v| fun(v)),
            Self::Dynamic(signal) => signal.try_with(|v| fun(v)),
        }
    }
}

impl<T> From<T> for JsMaybeSignal<T>
where
    T: 'static,
{
    fn from(value: T) -> Self {
        Self::Static(JsStoredValue::new(value))
    }
}

impl<T> From<JsSignal<T>> for JsMaybeSignal<T>
where
    T: 'static,
{
    fn from(signal: JsSignal<T>) -> Self {
        Self::Dynamic(signal)
    }
}

impl<T> From<JsReadSignal<T>> for JsMaybeSignal<T>
where
    T: 'static,
{
    fn from(signal: JsReadSignal<T>) -> Self {
        Self::Dynamic(signal.into())
    }
}