mod components;
pub(crate) mod core;

pub use components::*;
pub use core::IntoLatLng;

/// Leaflet re-exports
pub use leaflet;

use paste::paste;

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
