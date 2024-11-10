use leptos::*;
use wasm_bindgen::JsCast;

/// A context struct for the Leaflet map.
/// 
/// This struct provides a way to access the current state of the Leaflet map.
#[derive(Debug, Clone, Copy)]
pub struct LeafletMapContext {
    map: RwSignal<Option<leaflet::Map>>,
}

impl LeafletMapContext {
    /// Creates a new `LeafletMapContext` instance.
    pub fn new() -> Self {
        Self {
            map: create_rw_signal(None),
        }
    }

    /// Sets the map for the context.
    ///
    /// # Arguments
    ///
    /// * `map` - A reference to a `leaflet::Map` object.
    pub fn set_map(&self, map: &leaflet::Map) {
        self.map.set(Some(map.clone()));
    }

    /// Returns an optional `leaflet::Map` instance.
    pub fn map(&self) -> Option<leaflet::Map> {
        self.map.get()
    }

    /// Returns a signal that emits the current map instance.
    pub fn map_signal(&self) -> ReadSignal<Option<leaflet::Map>> {
        self.map.read_only()
    }

    /// Adds a layer to the context.
    ///
    /// # Arguments
    ///
    /// * `layer` - A reference to the layer to be added.
    ///
    pub fn add_layer<L: Into<leaflet::Layer> + Clone>(&self, layer: &L) {
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
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.to_owned().into();
        layer.remove_from(&map);
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
    container: RwSignal<Option<leaflet::Layer>>,
}

impl LeafletOverlayContainerContext {
    pub fn new() -> Self {
        Self {
            container: create_rw_signal(None),
        }
    }

    /// Calls set on the inner signal for the Layer
    pub fn set_container<C: Into<leaflet::Layer> + Clone>(&self, layer: &C) {
        self.container.set(Some(layer.clone().into()));
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

impl Default for LeafletOverlayContainerContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileLayerWmsContext {
    wms: RwSignal<Option<leaflet::TileLayerWms>>,
}

impl Default for TileLayerWmsContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TileLayerWmsContext {
    pub fn new() -> Self {
        Self {
            wms: create_rw_signal(None),
        }
    }

    pub fn set_wms(&self, wms: &leaflet::TileLayerWms) {
        self.wms.set(Some(wms.clone()));
    }

    pub fn wms(&self) -> Option<leaflet::TileLayerWms> {
        self.wms.get()
    }
}
