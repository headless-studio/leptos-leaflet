use leptos::*;

use crate::components::context::LeafletMapContext;

#[component(transparent)]
pub fn TileLayer(
    #[prop(into)] url: String,
    #[prop(into, optional)] attribution: String,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");

    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            let mut options = leaflet::TileLayerOptions::default();
            if attribution.is_empty() {
                options.attribution(&attribution);
            }
            let map_layer = leaflet::TileLayer::new_options(&url, &options);
            map_layer.addTo(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
