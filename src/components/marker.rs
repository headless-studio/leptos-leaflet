use crate::components::extend_context_with_overlay;
use leaflet::LatLng;
use leptos::*;

use super::LeafletMapContext;

#[component(transparent)]
pub fn Marker(
    cx: Scope,
    #[prop(into)] position: MaybeSignal<LatLng>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = position.clone();
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    let overlay = extend_context_with_overlay(cx);

    let cloned_overlay = overlay.clone();
    create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding marker");
            let marker = leaflet::Marker::new(&position());
            marker.addTo(&map);
            overlay.set_container(marker);
            // map_context.set_marker(marker);
        };
    });

    create_effect(cx, move |_| {
        position_tracking.track();
        if let Some(marker) = cloned_overlay.container::<leaflet::Marker>() {
            log!("Updating marker");
            marker.setLatLng(&position_tracking.get_untracked());
        }
    });

    let children = children
        .map(|children| {
            children(cx)
                .as_children()
                .iter()
                .map(|child| child.into_view(cx))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    view! {cx,
        {children}
    }
}
