use leptos::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy)]
pub struct LeafletMapContext {
    map: RwSignal<Option<leaflet::Map>>,
}

impl LeafletMapContext {
    pub fn new() -> Self {
        Self {
            map: create_rw_signal(None),
        }
    }

    pub fn set_map(&self, map: &leaflet::Map) {
        self.map.set(Some(map.clone()));
    }

    pub fn map(&self) -> Option<leaflet::Map> {
        self.map.get()
    }

    pub fn map_signal(&self) -> ReadSignal<Option<leaflet::Map>> {
        self.map.read_only()
    }

    pub fn add_layer<L: Into<leaflet::Layer> + Clone>(&self, layer: &L) {
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.to_owned().into();
        layer.add_to(&map);
    }

    pub fn remove_layer<L: Into<leaflet::Layer> + Clone>(&self, layer: &L) {
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.to_owned().into();
        layer.remove_from(&map);
    }
}

pub fn provide_leaflet_context() -> LeafletMapContext {
    let context = LeafletMapContext::new();
    provide_context(context);
    context
}

pub fn use_leaflet_context() -> Option<LeafletMapContext> {
    use_context::<LeafletMapContext>()
}

pub fn extend_context_with_overlay() -> LeafletOverlayContainerContext {
    let overlay_context = LeafletOverlayContainerContext::new();
    provide_context(overlay_context);
    overlay_context
}

pub fn use_overlay_context() -> Option<LeafletOverlayContainerContext> {
    use_context::<LeafletOverlayContainerContext>()
}

pub fn use_overlay_context_layer<T>() -> Option<T>
where
    T: Into<leaflet::Layer> + Clone + JsCast,
{
    expect_context::<LeafletOverlayContainerContext>().container::<T>()
}

pub fn update_overlay_context<C: Into<leaflet::Layer> + Clone>(layer: &C) {
    let overlay_context = use_context::<LeafletOverlayContainerContext>().expect("overlay context");
    overlay_context.set_container(layer);
}

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
