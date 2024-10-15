use leptos::logging::warn;
use leptos::prelude::*;

use crate::core::JsStoredValue;

use super::LeafletMapContext;

#[component(transparent)]
pub fn TileLayer(
    #[prop(into)] url: String,
    #[prop(into, optional)] attribution: String,
    #[prop(optional)] bring_to_front: bool,
    #[prop(optional)] bring_to_back: bool,
    #[prop(default = 0.0)] min_zoom: f64,
    #[prop(default = 18.0)] max_zoom: f64,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            let options = leaflet::TileLayerOptions::default();
            if !attribution.is_empty() {
                options.set_attribution(attribution.to_string());
            }
            options.set_min_zoom(min_zoom);
            options.set_max_zoom(max_zoom);
            let map_layer = leaflet::TileLayer::new_options(&url, &options);
            map_layer.add_to(&map);

            match (bring_to_front, bring_to_back) {
                (true, true) => warn!("The parameters are set to bring the layer to front and back at the same time. Ignoring these parameters..."),
                (true, false) => {map_layer.bring_to_front();}
                (false, true) => {map_layer.bring_to_back();}
                (false, false) => (),
            }

            let map_layer = JsStoredValue::new_local(map_layer);

            on_cleanup(move || {
                map_layer.with_value(|v| v.remove());
            });
        }
    });
}
