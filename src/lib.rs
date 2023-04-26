use components::Position;

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
