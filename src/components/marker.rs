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
    let mut map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    map_context.upgrade_marker_context(cx);

    create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding marker");
            let marker = leaflet::Marker::new(&position());
            marker.addTo(&map);
            map_context.set_marker(marker);
        };
    });
    create_effect(cx, move |_| {
        position_tracking.track();
        let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
        if let Some(marker) = map_context.marker() {
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
