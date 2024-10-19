/*
 * Copyright (c) HeadlessStudio  2023.
 */
mod leaflet_maybe_signal;

pub use leaflet_maybe_signal::LeafletMaybeSignal;

pub trait IntoLatLng {
    fn into_lat_lng(self) -> leaflet::LatLng;
}

impl IntoLatLng for (f64, f64) {
    fn into_lat_lng(self) -> leaflet::LatLng {
        leaflet::LatLng::new(self.0, self.1)
    }
}