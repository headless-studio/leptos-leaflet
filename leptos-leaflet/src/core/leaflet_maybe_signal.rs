/*
 * Copyright (c) HeadlessStudio  2023.
 */


use leptos::*;

#[derive(Copy, Clone)]
pub enum LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    Static(StoredValue<Option<T>>),
    Dynamic(Signal<Option<T>>),
}

impl<T> Default for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn default() -> Self {
        Self::Static(store_value(None))
    }
}

impl<T> SignalGet<Option<T>> for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn get(&self) -> Option<T> {
        match self {
            LeafletMaybeSignal::Static(v) => v.get_value(),
            LeafletMaybeSignal::Dynamic(v) => v.get(),
        }
    }

    fn try_get(&self) -> Option<Option<T>> {
        match self {
            LeafletMaybeSignal::Static(v) => v.try_get_value(),
            LeafletMaybeSignal::Dynamic(v) => v.try_get(),
        }
    }
}

impl<T> SignalWith<Option<T>> for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> O {
        match self {
            LeafletMaybeSignal::Static(v) => f(&v.get_value()),
            LeafletMaybeSignal::Dynamic(v) => v.with(f),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match self {
            LeafletMaybeSignal::Static(v) => Some(f(&v.get_value())),
            LeafletMaybeSignal::Dynamic(v) => v.try_with(f),
        }
    }
}

impl<T> SignalWithUntracked<Option<T>> for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> O {
        match self {
            LeafletMaybeSignal::Static(v) => f(&v.get_value()),
            LeafletMaybeSignal::Dynamic(v) => v.with_untracked(f),
        }
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Option<T>) -> O) -> Option<O> {
        match self {
            LeafletMaybeSignal::Static(v) => Some(f(&v.get_value())),
            LeafletMaybeSignal::Dynamic(v) => v.try_with_untracked(f),
        }
    }
}

impl<T> SignalGetUntracked<Option<T>> for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn get_untracked(&self) -> Option<T> {
        match self {
            LeafletMaybeSignal::Static(v) => v.get_value(),
            LeafletMaybeSignal::Dynamic(v) => v.get_untracked(),
        }
    }

    fn try_get_untracked(&self) -> Option<Option<T>> {
        match self {
            LeafletMaybeSignal::Static(v) => v.try_get_value(),
            LeafletMaybeSignal::Dynamic(v) => v.try_get_untracked(),
        }
    }
}

impl<T> From<Option<T>> for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn from(target: Option<T>) -> Self {
        LeafletMaybeSignal::Static(store_value(target))
    }
}

impl<T> From<T> for LeafletMaybeSignal<T>
    where
        T: Clone + 'static,
{
    fn from(target: T) -> Self {
        LeafletMaybeSignal::Static(store_value(Some(target)))
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

impl_from_signal_option!(Signal<Option<T>>);
impl_from_signal_option!(ReadSignal<Option<T>>);
impl_from_signal_option!(RwSignal<Option<T>>);
impl_from_signal_option!(Memo<Option<T>>);

macro_rules! impl_from_signal {
    ($ty:ty) => {
        impl<T> From<$ty> for LeafletMaybeSignal<T>
        where
            T: Clone + 'static,
        {
            fn from(target: $ty) -> Self {
                let signal = target;

                Self::Dynamic(Signal::derive(move || Some(signal.get())))
            }
        }
    };
}

impl_from_signal!(Signal<T>);
impl_from_signal!(ReadSignal<T>);
impl_from_signal!(RwSignal<T>);
impl_from_signal!(Memo<T>);

impl From<&str> for LeafletMaybeSignal<String> {
    fn from(value: &str) -> Self {
        Self::Static(store_value(Some(value.to_string())))
    }
}