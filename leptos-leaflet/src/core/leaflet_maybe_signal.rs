/*
 * Copyright (c) HeadlessStudio  2023.
 */

use std::panic::Location;

use leptos::prelude::*;

use super::{JsReadSignal, JsRwSignal, JsSignal, JsStoredValue};

#[derive(Copy, Clone)]
pub enum LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    Static(JsStoredValue<Option<T>>),
    Dynamic(JsSignal<Option<T>>),
}

impl<T> Default for LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    fn default() -> Self {
        Self::Static(JsStoredValue::new(None))
    }
}

impl<T> DefinedAt for LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        match self {
            LeafletMaybeSignal::Static(v) => v.defined_at(),
            LeafletMaybeSignal::Dynamic(v) => v.defined_at(),
        }
    }
}

impl<T> With for LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    type Value = Option<T>;

    fn try_with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        match self {
            LeafletMaybeSignal::Static(v) => v.try_with_value(|v| fun(v)),
            LeafletMaybeSignal::Dynamic(v) => v.try_with(|v| fun(v)),
        }
    }
}

impl<T> WithUntracked for LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    type Value = Option<T>;

    fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        match self {
            LeafletMaybeSignal::Static(v) => v.try_with_value(|v| fun(v)),
            LeafletMaybeSignal::Dynamic(v) => v.try_with_untracked(|v| fun(v)),
        }
    }
}

impl<T> From<Option<T>> for LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    fn from(target: Option<T>) -> Self {
        LeafletMaybeSignal::Static(JsStoredValue::new(target))
    }
}

impl<T> From<T> for LeafletMaybeSignal<T>
where
    T: Clone + 'static,
{
    fn from(target: T) -> Self {
        LeafletMaybeSignal::Static(JsStoredValue::new(Some(target)))
    }
}

macro_rules! impl_from_signal_option {
    ($ty:ty) => {
        impl<T> From<$ty> for LeafletMaybeSignal<T>
        where
            T: Clone + 'static,
        {
            fn from(target: $ty) -> Self {
                Self::Dynamic(target.into())
            }
        }
    };
}

impl_from_signal_option!(JsSignal<Option<T>>);
impl_from_signal_option!(JsReadSignal<Option<T>>);
impl_from_signal_option!(JsRwSignal<Option<T>>);