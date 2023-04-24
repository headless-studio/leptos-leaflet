mod map_container;
mod marker;
mod popup;
mod tile_layer;

use leptos::*;
pub use map_container::MapContainer;
pub use marker::Marker;
pub use popup::Popup;
pub use tile_layer::TileLayer;

#[derive(Debug, Clone)]
pub struct LeafletMapContext {
    map: RwSignal<Option<leaflet::Map>>,
    marker: RwSignal<Option<leaflet::Marker>>,
    in_marker: RwSignal<Vec<bool>>,
}

impl LeafletMapContext {
    pub fn new(cx: Scope) -> Self {
        log!("Creating map context");
        Self {
            map: create_rw_signal(cx, None),
            marker: create_rw_signal(cx, None),
            in_marker: create_rw_signal(cx, vec![]),
        }
    }

    pub fn set_map(&self, map: leaflet::Map) {
        self.map.set(Some(map));
    }

    pub fn upgrade_marker_context(&mut self, cx: Scope) {
        self.in_marker.update(|u| u.push(true));
        on_cleanup(cx, move || {
            log!("Cleaning up marker context");
            use_context::<LeafletMapContext>(cx)
                .expect("not on a leaflet context")
                .in_marker
                .update(|u| {u.pop();});
        });
    }

    pub fn in_marker(&self) -> bool {
        !self.in_marker.get().is_empty()
    }

    pub fn set_marker(&self, marker: leaflet::Marker) {
        self.marker.set(Some(marker));
    }

    pub fn map(&self) -> Option<leaflet::Map> {
        self.map.get()
    }

    pub fn map_signal(&self) -> ReadSignal<Option<leaflet::Map>> {
        self.map.read_only()
    }

    pub fn marker(&self) -> Option<leaflet::Marker> {
        self.marker.get()
    }

    pub fn marker_signal(&self) -> ReadSignal<Option<leaflet::Marker>> {
        self.marker.read_only()
    }
}
