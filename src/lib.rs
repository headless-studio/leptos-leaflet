pub mod components;
pub mod core;

#[macro_export]
macro_rules! position {
    ($lat: expr, $lng: expr) => {
        MaybeSignal::Static($crate::components::Position::new($lat, $lng))
    };
}