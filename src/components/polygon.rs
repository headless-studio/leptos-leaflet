use leaflet::{PolylineOptions, to_lat_lng_array};
use leptos::*;

use crate::components::{extend_context_with_overlay, update_overlay_context, LeafletMapContext, Position, LeafletOverlayContainerContext};

#[component(transparent)]
pub fn Polygon(
    cx: Scope,
    #[prop(into, optional)] options: PolylineOptions,
    #[prop(into)] positions: MaybeSignal<Vec<Position>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    cx.child_scope(|cx| {
        extend_context_with_overlay(cx);

        let positions_for_effect = positions.clone();
        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let lat_lngs = to_lat_lng_array(&positions.get_untracked());
                let polygon = leaflet::Polygon::new_with_options(&lat_lngs, &options);
                polygon.addTo(&map);
                update_overlay_context(cx, &polygon);
                on_cleanup(cx, move || {
                    polygon.remove();
                });
            };
        });

        create_effect(cx, move |_| {
            if let Some(polygon) = use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context").container::<leaflet::Polygon>() {
                let lat_lngs = to_lat_lng_array(&positions_for_effect());
                polygon.setLatLngs(&lat_lngs);
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
