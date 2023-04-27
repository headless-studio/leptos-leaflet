use crate::components::LeafletMapContext;
use leaflet::ImageOverlayOptions;
use leptos::*;

#[component(transparent)]
pub fn ImageOverlay(
    cx: Scope,
    #[prop(into)] url: String,
    #[prop(into)] bounds: leaflet::LatLngBounds,
    #[prop(into, optional)] attribution: String,
    #[prop(into, optional)] options: ImageOverlayOptions,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("map context not found");
    create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding image layer: {}", url);
            let mut options = options.clone();
            if !attribution.is_empty() {
                options.attribution(&attribution);
            }
            let map_layer = leaflet::ImageOverlay::new_with_options(&url, &bounds, &options);
            map_layer.addTo(&map);
            on_cleanup(cx, move || {
                map_layer.remove();
            });
        }
    });
}
