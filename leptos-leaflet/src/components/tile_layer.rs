use leaflet::TileLayerOptions;
use leptos::*;

use crate::components::context::LeafletMapContext;

#[component(transparent)]
pub fn TileLayer(
    cx: Scope,
    #[prop(into)] url: String,
    #[prop(into, optional)] attribution: String,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("map context not found");

    create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding tile layer: {}", url);
            let mut options = leaflet::TileLayerOptions::default();
            if attribution.is_empty() {
                options.attribution(&attribution);
            }
            let map_layer = leaflet::TileLayer::new_options(&url, &options);
            map_layer.addTo(&map);
            on_cleanup(cx, move || {
                map_layer.remove();
            });
        }
    });
}
