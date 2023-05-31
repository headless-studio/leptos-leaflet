use leptos::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy)]
pub struct LeafletMapContext {
    map: RwSignal<Option<leaflet::Map>>,
}

impl LeafletMapContext {
    pub fn new(cx: Scope) -> Self {
        log!("Creating map context");
        Self {
            map: create_rw_signal(cx, None),
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

    pub fn add_layer<L: Into<leaflet::Layer>>(&self, layer: &L) {
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.into();
        layer.addTo(&map);
    }

    pub fn remove_layer<L: Into<leaflet::Layer>>(&self, layer: &L) {
        let map = self.map.get_untracked().expect("Map to be available");
        let layer: leaflet::Layer = layer.into();
        layer.removeFrom(&map);
    }
}

pub fn provide_leaflet_context(cx: Scope) {
    provide_context(cx, LeafletMapContext::new(cx));
}

pub fn use_leaflet_context(cx: Scope) -> Option<LeafletMapContext> {
    use_context::<LeafletMapContext>(cx)
}

pub fn extend_context_with_overlay(cx: Scope) -> LeafletOverlayContainerContext {
    let overlay_context = LeafletOverlayContainerContext::new(cx);
    provide_context(cx, overlay_context.clone());
    overlay_context
}

pub fn use_overlay_context(cx: Scope) -> Option<LeafletOverlayContainerContext> {
    use_context::<LeafletOverlayContainerContext>(cx)
}

pub fn use_overlay_context_layer<T>(cx: Scope) -> Option<T>
where
    T: Into<leaflet::Layer> + Clone + JsCast,
{
    expect_context::<LeafletOverlayContainerContext>(cx).container::<T>()
}

pub fn update_overlay_context<C: Into<leaflet::Layer> + Clone>(cx: Scope, layer: &C) {
    let overlay_context =
        use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
    overlay_context.set_container(layer);
}

#[derive(Debug, Clone, Copy)]
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
