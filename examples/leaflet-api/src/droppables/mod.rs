use std::{any::Any, ops::Deref};

use leptos::logging::log;
use leptos_leaflet::{leaflet::*, prelude::*};

pub const DRAG_END_EVENT: &str = "dragend";
pub const DRAG_EVENT: &str = "drag";

pub trait EventHandler: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> EventHandler for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct DroppableMarker {
    marker: Marker,
    events: Vec<Box<dyn Any>>,
}

impl DroppableMarker {
    pub fn new(position: impl IntoLatLng, options: Option<MarkerOptions>) -> Self {
        let marker = if let Some(options) = options {
            Marker::new_with_options(&position.into_lat_lng(), &options)
        } else {
            Marker::new(&position.into_lat_lng())
        };
        Self { marker, events: Vec::new() }
    }

    pub fn add_event<E>(&mut self, event: E) -> &mut Self
    where
        E: EventHandler + 'static,
    {
        let event = Box::new(event);
        self.events.push(event);
        self
    }

    pub fn marker_ref(&self) -> &Marker {
        &self.marker
    }

    pub fn marker(&self) -> Marker {
        // This is just a pointer to the js object, not a clone of it
        self.marker.clone()
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
        log!("Dropping marker and removing events");
        self.marker.remove();
    }
}
