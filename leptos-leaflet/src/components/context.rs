use leaflet::Map;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::core::{JsReadSignal, JsRwSignal, JsWriteSignal};


/// A context struct for the Leaflet map.
/// 
/// This struct provides a way to access the current state of the Leaflet map.
#[derive(Debug, Clone, Copy)]
pub struct LeafletMapContext {
    map: JsRwSignal<Option<leaflet::Map>>,
    thread_id: std::thread::ThreadId,
}

impl LeafletMapContext {
    /// Creates a new `LeafletMapContext` instance.
    pub fn new() -> Self {
        Self {
            map: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        }
    }

    /// Sets the map for the context.
    ///
    /// # Arguments
    ///
    /// * `map` - A reference to a `leaflet::Map` object.
    pub fn set_map(&self, map: &leaflet::Map) {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return;
        }
        self.map.set(Some(map.clone()));
    }

    /// Returns an optional `leaflet::Map` instance.
    pub fn map(&self) -> Option<leaflet::Map> {
        if self.is_valid() {
            self.map.get()
        } else {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            None
        }
    }

    pub fn map_untracked(&self) -> Option<leaflet::Map> {
        if self.is_valid() {
            self.map.get_untracked()
        } else {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            None
        }
    }

    /// Returns a signal that emits the current map instance.
    pub fn map_signal(&self) -> JsReadSignal<Option<leaflet::Map>> {
        if self.is_valid() {
            self.map.read_only()
        } else {
            panic!("Accessing map from a different thread. Probably running on the server.");
        }
    }

    /// Adds a layer to the context.
    ///
    /// # Arguments
    ///
    /// * `layer` - A reference to the layer to be added.
    ///
    pub fn add_layer<L: Into<leaflet::Layer> + Clone>(&self, layer: &L) {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return;
        }
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.to_owned().into();
        layer.add_to(&map);
    }

    /// Removes a layer from the context.
    ///
    /// # Arguments
    ///
    /// * `layer` - A reference to the layer to be removed.
    ///
    pub fn remove_layer<L: Into<leaflet::Layer> + Clone>(&self, layer: &L) {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return;
        }
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.to_owned().into();
        layer.remove_from(&map);
    }

    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.map.is_disposed()
    }
}

impl Default for LeafletMapContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Provides the Leaflet map context.
pub fn provide_leaflet_context() -> LeafletMapContext {
    let context = LeafletMapContext::new();
    provide_context(context);
    context
}

/// Returns an optional `LeafletMapContext` object that can be used to access the current state of the Leaflet map.
pub fn use_leaflet_context() -> Option<LeafletMapContext> {
    use_context::<LeafletMapContext>()
}

/// Extends the context with an overlay container.
pub fn extend_context_with_overlay() -> LeafletOverlayContainerContext {
    let overlay_context = LeafletOverlayContainerContext::new();
    provide_context(overlay_context);
    overlay_context
}

/// Returns an optional `LeafletOverlayContainerContext` for the current overlay.
pub fn use_overlay_context() -> Option<LeafletOverlayContainerContext> {
    use_context::<LeafletOverlayContainerContext>()
}

/// Returns an optional `leafet::Layer` for the overlay context layer.
pub fn use_overlay_context_layer<T>() -> Option<T>
where
    T: Into<leaflet::Layer> + Clone + JsCast,
{
    expect_context::<LeafletOverlayContainerContext>().container::<T>()
}

/// Updates the overlay context with the given layer.
///
/// # Arguments
///
/// * `layer` - A cloneable object that can be converted into a `leaflet::Layer`.
pub fn update_overlay_context<C: Into<leaflet::Layer> + Clone>(layer: &C) {
    let overlay_context = use_context::<LeafletOverlayContainerContext>().expect("overlay context");
    overlay_context.set_container(layer);
}

/// A context struct for Leaflet overlay container.
#[derive(Debug, Clone, Copy)]
pub struct LeafletOverlayContainerContext {
    container: JsRwSignal<Option<leaflet::Layer>>,
    thread_id: std::thread::ThreadId,
}

impl LeafletOverlayContainerContext {
    pub fn new() -> Self {
        Self {
            container: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        }
    }

    /// Calls set on the inner signal for the Layer
    pub fn set_container<C: Into<leaflet::Layer> + Clone>(&self, layer: &C) {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return;
        }
        self.container.set(Some(layer.clone().into()));
    }

    /// Calls get on the inner signal for the Layer
    pub fn container<T: JsCast>(&self) -> Option<T> {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return None;
        }
        self.container.get().map(|layer| layer.unchecked_into())
    }

    /// Calls get_untracked on the inner signal for the Layer
    pub fn untrack_container<C: JsCast>(&self) -> Option<C> {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return None;
        }
        self.container
            .get_untracked()
            .map(|layer| layer.unchecked_into())
    }

    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.container.is_disposed()
    }
}

impl Default for LeafletOverlayContainerContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileLayerWmsContext {
    wms: JsRwSignal<Option<leaflet::TileLayerWms>>,
    thread_id: std::thread::ThreadId,
}

impl Default for TileLayerWmsContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TileLayerWmsContext {
    pub fn new() -> Self {
        Self {
            wms: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        }
    }

    pub fn set_wms(&self, wms: &leaflet::TileLayerWms) {
        if !self.is_valid() {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            return;
        }
        self.wms.set(Some(wms.clone()));
    }

    pub fn wms(&self) -> Option<leaflet::TileLayerWms> {
        if self.is_valid() {
            self.wms.get()
        } else {
            leptos::logging::error!("Accessing map from a different thread. Probably running on the server.");
            None
        }
    }

    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.wms.is_disposed()
    }
}

pub type MapReadSignal = JsReadSignal<Option<Map>>;
pub type MapWriteSignal = JsWriteSignal<Option<Map>>;

/// Creates a pair of signals for reading and writing a `leaflet::Map` instance.
pub fn create_map_signal() -> (MapReadSignal, MapWriteSignal) {
    JsRwSignal::new_local(None).split()
}
