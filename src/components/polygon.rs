use leaflet::{PolylineOptions, to_lat_lng_array};
use leptos::*;

use crate::components::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext, Position,
};
use wasm_bindgen::prelude::*;

#[component]
pub fn Polygon(
    cx: Scope,
    #[prop(into, optional)] options: MaybeSignal<PolylineOptions>,
    #[prop(into)] positions: Vec<Position>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    cx.child_scope(|cx| {
        extend_context_with_overlay(cx);

        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                log!("Adding polygon");
                let lat_lngs = to_lat_lng_array(&positions);
                let polygon = leaflet::Polygon::new_with_options(&lat_lngs, &options.get_untracked());
                polygon.addTo(&map);
                update_overlay_context(cx, &polygon);
                on_cleanup(cx, move || {
                    polygon.remove();
                });
            };
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
