use leaflet::{LatLng, PolylineOptions, to_lat_lng_array};
use leptos::*;

use wasm_bindgen::prelude::*;

use crate::components::{LeafletMapContext, LeafletOverlayContainerContext};

use super::{extend_context_with_overlay, update_overlay_context, Position};

#[component(transparent)]
pub fn Polyline(
    cx: Scope,
    #[prop(into)]
    positions: MaybeSignal<Vec<Position>>,
    #[prop(into, optional)]
    options: PolylineOptions,
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    cx.child_scope(|cx| {
        extend_context_with_overlay(cx);
        
        let positions_for_effect = positions.clone();
        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                log!("Adding polyline");
                let lat_lngs = to_lat_lng_array(&positions.get_untracked());
                let polyline = leaflet::Polyline::new_with_options(&lat_lngs, &options);
                polyline.addTo(&map);
                update_overlay_context(cx, &polyline);
                on_cleanup(cx, move ||{
                    polyline.remove();
                });
            }
        });
        
        create_effect(cx, move |_| {
           if let Some(polyline) = use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context").container::<leaflet::Polyline>() {
               let lat_lngs = to_lat_lng_array(&positions_for_effect());
               polyline.setLatLngs(&lat_lngs);
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
