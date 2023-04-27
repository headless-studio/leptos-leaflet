use leaflet::MarkerOptions;
use super::{extend_context_with_overlay, Position};

use leptos::*;

use super::LeafletMapContext;

#[component(transparent)]
pub fn Marker(
    cx: Scope,
    #[prop(into)] position: MaybeSignal<Position>,
    #[prop(into, optional)] draggable: MaybeSignal<bool>,
    #[prop(into, optional)] options: MarkerOptions,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = position.clone();
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");

    cx.child_scope(|cx| {
        let overlay = extend_context_with_overlay(cx);
        create_effect(cx, move |_| {
            if let Some(map) = map_context.map() {
                log!("Adding marker");
                let mut options = options.clone();
                let drag = draggable.get_untracked();
                if drag {
                    options.draggable(drag);
                }
                let marker = leaflet::Marker::new_with_options(&position.get_untracked().into(), &options);
                marker.addTo(&map);
                overlay.set_container(&marker);

                on_cleanup(cx, move || {
                    marker.remove();
                });
            };
        });

        create_effect(cx, move |_| {
            position_tracking.track();
            if let Some(marker) = overlay.container::<leaflet::Marker>() {
                log!("Updating marker");
                marker.setLatLng(&position_tracking.get_untracked().into());
            }
        });
        
        create_effect(cx, move |_| {
            if let Some(marker) = overlay.container::<leaflet::Marker>() {
                log!("Changing draggable");
                if draggable.get() {
                    marker.dragging().enable();
                } else {
                    marker.dragging().disable();
                }
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
