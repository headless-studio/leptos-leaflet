pub mod components;
pub mod core;

#[macro_export]
macro_rules! latlng {
    ($lat: expr, $lng: expr) => {
        MaybeSignal::Static(($lat, $lng).into())
    };
}