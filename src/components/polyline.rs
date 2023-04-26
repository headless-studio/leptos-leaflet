use leaflet::{LatLng, PolylineOptions, to_lat_lng_array};
use leptos::*;

use wasm_bindgen::prelude::*;

use crate::components::LeafletMapContext;

use super::{extend_context_with_overlay, update_overlay_context, Position};

#[component]
pub fn Polyline(
    cx: Scope,
    #[prop(into, optional)]
    options: MaybeSignal<PolylineOptions>,
    #[prop(into)]
    positions: Vec<Position>,
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    cx.child_scope(|cx| {
        extend_context_with_overlay(cx);

        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                log!("Adding polyline");
                let lat_lngs = to_lat_lng_array(&positions);
                let polyline = leaflet::Polyline::new_with_options(&lat_lngs, &options());
                polyline.addTo(&map);
                update_overlay_context(cx, &polyline);
                on_cleanup(cx, move ||{
                    polyline.remove();
                });
            }
            
        });

        children
        .map(|children| {
            children(cx)
                .as_children()
                .iter()
                .map(|child| child.into_view(cx))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    });
}
