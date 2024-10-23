use std::ops::Deref;

use leptos_leaflet::{leaflet::*, IntoLatLng};

pub struct DroppableMarker {
    marker: Marker
}

impl DroppableMarker {
    pub fn new(position: impl IntoLatLng, options: Option<MarkerOptions>) -> Self {
        let marker = if let Some(options) = options {
            Marker::new_with_options(&position.into_lat_lng(), &options)
        } else {
            Marker::new(&position.into_lat_lng())
        };
        Self { marker }
    }
}

impl Deref for DroppableMarker {
    type Target = Marker;

    fn deref(&self) -> &Self::Target {
        &self.marker
    }
}

impl Drop for DroppableMarker {
    fn drop(&mut self) {
        self.marker.remove();
    }
}