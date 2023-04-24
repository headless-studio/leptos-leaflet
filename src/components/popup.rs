use leptos::*;
use wasm_bindgen::prelude::*;

use super::LeafletMapContext;

#[component]
pub fn Popup(
    cx: Scope,
    #[prop(into, optional)] position: MaybeSignal<leaflet::LatLng>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");

    let content = view! {cx, <div>{children(cx)}</div>};
    create_effect(cx, move |_| {
        log!("Popup context {:?}", map_context);
        let inner_content = content.clone();
        map_context.marker_signal().track();
        if map_context.in_marker() {
            log!("we are in marker");
            if let (Some(marker), Some(map)) = (map_context.marker(), map_context.map()) {
                log!("Adding popup");
                let options = leaflet::PopupOptions::default();
                let popup = leaflet::Popup::new(&options, Some(marker.unchecked_ref()));
                let html_view: &JsValue = inner_content.unchecked_ref();
                popup.setContent(html_view);
                popup.setLatLng(&marker.getLatLng());
                marker.bindPopup(&popup);
            }
        } else if let Some(map) = map_context.map() {
            log!("Adding popup");
            let options = leaflet::PopupOptions::default();
            let popup = leaflet::Popup::new_with_lat_lng(&position(), &options);
            let html_view: &JsValue = inner_content.unchecked_ref();
            popup.setContent(html_view);
            popup.openOn(&map);
        }
    });

    view! {cx,}
}
