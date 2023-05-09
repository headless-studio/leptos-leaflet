mod components;
pub use components::*;
pub use leaflet::{
    ErrorEvent, Event, LatLng, LatLngBounds, LocationEvent, MouseEvent, PopupEvent, TooltipEvent,
};

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
