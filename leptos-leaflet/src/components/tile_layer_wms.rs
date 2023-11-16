use leptos::*;

use crate::components::context::LeafletMapContext;
use leaflet::TileLayerWmsOptions;

#[component(transparent)]
pub fn TileLayerWms(#[prop(into)] url: String, options: TileLayerWmsOptions) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");

    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            let map_layer = leaflet::TileLayerWms::new_options(&url, &options);
            map_layer.add_to(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
