mod map_container;
mod marker;
mod popup;
mod tile_layer;
mod tooltip;

use leaflet::LatLng;
use leptos::*;
pub use map_container::MapContainer;
pub use marker::Marker;
pub use popup::Popup;
pub use tile_layer::TileLayer;
pub use tooltip::Tooltip;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
pub struct LeafletMapContext {
    map: RwSignal<Option<leaflet::Map>>,
}

pub fn provide_leaflet_context(cx: Scope) {
    provide_context(cx, LeafletMapContext::new(cx));
}

impl LeafletMapContext {
    pub fn new(cx: Scope) -> Self {
        log!("Creating map context");
        Self {
            map: create_rw_signal(cx, None),
        }
    }

    pub fn set_map(&self, map: leaflet::Map) {
        self.map.set(Some(map));
    }

    pub fn map(&self) -> Option<leaflet::Map> {
        self.map.get()
    }

    pub fn map_signal(&self) -> ReadSignal<Option<leaflet::Map>> {
        self.map.read_only()
    }
}

pub fn extend_context_with_overlay(cx: Scope) -> LeafletOverlayContainerContext {
    let overlay_context = LeafletOverlayContainerContext::new(cx);
    provide_context(cx, overlay_context.clone());
    overlay_context
}

#[derive(Debug, Clone)]
pub struct LeafletOverlayContainerContext {
    container: RwSignal<Option<leaflet::Layer>>,
}

impl LeafletOverlayContainerContext {
    pub fn new(cx: Scope) -> Self {
        Self {
            container: create_rw_signal(cx, None),
        }
    }

    /// Calls set on the inner signal for the Layer
    pub fn set_container<C: Into<leaflet::Layer>>(&self, layer: C) {
        self.container.set(Some(layer.into()));
    }

    /// Calls get on the inner signal for the Layer
    pub fn container<T: JsCast>(&self) -> Option<T> {
        self.container.get().map(|layer| layer.unchecked_into())
    }

    /// Calls get_untracked on the inner signal for the Layer
    pub fn untrack_container<C: JsCast>(&self) -> Option<C> {
        self.container
            .get_untracked()
            .map(|layer| layer.unchecked_into())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

impl Position {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

impl From<Position> for LatLng {
    fn from(value: Position) -> Self {
        LatLng::new(value.lat, value.lng)
    }
}

impl From<Position> for (f64, f64) {
    fn from(value: Position) -> Self {
        (value.lat, value.lng)
    }
}

impl From<Position> for [f64; 2] {
    fn from(value: Position) -> Self {
        [value.lat, value.lng]
    }
}