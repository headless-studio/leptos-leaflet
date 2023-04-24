use leptos::*;
use wasm_bindgen::{JsCast, JsValue};

use super::LeafletMapContext;

#[component]
pub fn Popup(cx: Scope, #[prop(into)] content: String) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");

    create_effect(cx, move |_| {
        log!("Popup context {:?}", map_context);
        map_context.marker_signal().track();
        if map_context.in_marker() {
            log!("we are in marker");
            if let (Some(marker), Some(map)) = (map_context.marker(), map_context.map()) {
                let content = content.clone();
                log!("Adding popup");
                let options = leaflet::PopupOptions::default();
                let popup = leaflet::Popup::new(
                    &options,
                    Some(marker.unchecked_ref()),
                );
                popup.setContent(&content.into());
                popup.setLatLng(&marker.getLatLng());
                popup.openOn(&map);
                // marker.bindPopup(&popup);
            }
        }
    });

    view! {cx,
    }
}
