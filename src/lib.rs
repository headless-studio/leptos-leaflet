use components::Position;
use leptos::{MaybeSignal, RwSignal, Signal, SignalGetUntracked};
use std::ops::Deref;

pub mod components;
pub mod core;

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

#[derive(Clone, Debug, Default)]
pub struct MaybeSignalString {
    value: MaybeSignal<Option<String>>,
}

impl MaybeSignalString {
    pub fn new(value: &str) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value.to_string())),
        }
    }
}

impl Deref for MaybeSignalString {
    type Target = MaybeSignal<Option<String>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<&str> for MaybeSignalString {
    fn from(value: &str) -> Self {
        Self {
            value: MaybeSignal::Static(Some(value.to_string())),
        }
    }
}

impl From<Signal<Option<String>>> for MaybeSignalString {
    fn from(value: Signal<Option<String>>) -> Self {
        Self { value: value.into() }
    }
}