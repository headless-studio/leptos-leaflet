use leptos::*;

use super::LeafletMapContext;

#[component(transparent)]
pub fn TileLayer(cx: Scope, 
    #[prop(into)]
    url: String,
    #[prop(into, optional)]
    attribution: String,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("map context not found");
    let add_layer = create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding tile layer: {}", url);
            let map_layer = leaflet::TileLayer::new(&url);
            map_layer.addTo(&map);
            on_cleanup(cx, move || {
                map_layer.remove();
            });

        }
    });
    view! { cx,
        {add_layer}
    }
}
