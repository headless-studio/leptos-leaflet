use components::Position;
use leptos::{
    MaybeSignal, SignalGet, SignalGetUntracked, SignalWith, SignalWithUntracked,
};
use std::ops::Deref;

pub mod components;

#[macro_export]
macro_rules! position {
    ($lat: expr, $lng: expr) => {
        MaybeSignal::Static($crate::components::Position::new($lat, $lng))
    };
}

#[macro_export]
macro_rules! pos_opt {
    ($lat: expr, $lng: expr) => {
        MaybeSignal::Static(Some($crate::components::Position::new($lat, $lng)))
    };
}

pub fn positions(positions: &[(f64, f64)]) -> Vec<Position> {
    positions
        .iter()
        .map(|&(lat, lng)| Position::new(lat, lng))
        .collect()
}

#[derive(Debug)]
pub struct MaybeSignalOption<T>
where
    T: 'static,
{
    value: MaybeSignal<Option<T>>,
}

impl<T: Clone> Clone for MaybeSignalOption<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: Copy> Copy for MaybeSignalOption<T> {}

impl<T: Default> Default for MaybeSignalOption<T> {
    fn default() -> Self {
        MaybeSignalOption {
            value: MaybeSignal::Static(None),
        }
    }
}

impl<T: Clone> MaybeSignalOption<T> {
    fn get(&self) -> Option<T> {
        self.value.get()
    }

    fn try_get(&self) -> Option<Option<T>> {
        self.value.try_get()
    }

    fn get_untracked(&self) -> Option<T> {
        self.value.get_untracked()
    }

    fn try_get_untracked(&self) -> Option<Option<T>> {
        self.value.try_get_untracked()
    }
}

impl MaybeSignalOption<String> {
    pub fn new(value: &str) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value.to_string())),
        }
    }
}

impl Deref for MaybeSignalOption<String> {
    type Target = MaybeSignal<Option<String>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<&str> for MaybeSignalOption<String> {
    fn from(value: &str) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value.to_string())),
        }
    }
}

impl From<bool> for MaybeSignalOption<bool> {
    fn from(value: bool) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value)),
        }
    }
}

impl From<f64> for MaybeSignalOption<f64> {
    fn from(value: f64) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value)),
        }
    }
}

impl From<(u32, u32)> for MaybeSignalOption<(u32, u32)> {
    fn from(value: (u32, u32)) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value)),
        }
    }
}
