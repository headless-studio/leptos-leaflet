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
            let options = leaflet::TileLayerOptions::default();
            if !attribution.is_empty() {
                options.set_attribution(attribution.to_string());
            }
            let map_layer = leaflet::TileLayer::new_options(&url, &options);
            map_layer.add_to(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
