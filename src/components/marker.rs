use super::{extend_context_with_overlay, Position};

use leptos::*;

use super::LeafletMapContext;

#[component(transparent)]
pub fn Marker(
    cx: Scope,
    #[prop(into)] position: MaybeSignal<Position>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = position.clone();
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");

    cx.child_scope(|cx| {
        let overlay = extend_context_with_overlay(cx);
        let cloned_overlay = overlay.clone();
        create_effect(cx, move |_| {
            if let Some(map) = map_context.map() {
                log!("Adding marker");
                let marker = leaflet::Marker::new(&position().into());
                marker.addTo(&map);
                overlay.set_container(&marker);

                on_cleanup(cx, move || {
                    marker.remove();
                });
            };
        });

        create_effect(cx, move |_| {
            position_tracking.track();
            if let Some(marker) = cloned_overlay.container::<leaflet::Marker>() {
                log!("Updating marker");
                marker.setLatLng(&position_tracking.get_untracked().into());
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
