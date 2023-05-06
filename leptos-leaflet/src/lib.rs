mod components;
pub use components::*;
use leptos::{MaybeSignal, SignalGet, SignalGetUntracked};
use std::ops::Deref;

pub use leaflet::{ErrorEvent, Event, LocationEvent, PopupEvent};

#[macro_export]
macro_rules! position {
    ($lat: expr, $lng: expr) => {
        MaybeSignal::Static($crate::Position::new($lat, $lng))
    };
}

pub fn positions(positions: &[(f64, f64)]) -> Vec<Position> {
    positions
        .iter()
        .map(|&(lat, lng)| Position::new(lat, lng))
        .collect()
}
