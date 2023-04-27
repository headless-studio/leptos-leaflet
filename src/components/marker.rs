use super::{extend_context_with_overlay, Position};
use leptos::*;

use super::LeafletMapContext;

#[component(transparent)]
pub fn Marker(
    cx: Scope,
    /// Position for the Marker
    #[prop(into)] position: MaybeSignal<Position>,
    #[prop(into, optional)] draggable: MaybeSignal<bool>,
    #[prop(into, optional)] keyboard: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] title: MaybeSignal<Option<String>>,
    #[prop(into, optional)] alt: MaybeSignal<Option<String>>,
    #[prop(into, optional)] interactive: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] z_index_offset: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] rise_on_hover: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] rise_offset: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] pane: MaybeSignal<Option<String>>,
    #[prop(into, optional)] shadow_pane: MaybeSignal<Option<String>>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] auto_pan: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] auto_pan_padding: MaybeSignal<Option<(u32, u32)>>,
    #[prop(into, optional)] auto_pan_speed: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] icon_url: MaybeSignal<Option<String>>,
    #[prop(into, optional)] icon_size: MaybeSignal<Option<(u32, u32)>>,
    #[prop(into, optional)] attribution: MaybeSignal<Option<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = position.clone();
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");

    let (child, _) = cx.run_child_scope(|cx| {
        let overlay = extend_context_with_overlay(cx);
        create_effect(cx, move |_| {
            if let Some(map) = map_context.map() {
                log!("Adding marker");
                let mut options = leaflet::MarkerOptions::new();
                let drag = draggable.get_untracked();
                if drag {
                    options.draggable(drag);
                }
                if let Some(keyboard) = keyboard.get_untracked() {
                    options.keyboard(keyboard);
                }
                if let Some(title) = title.get_untracked() {
                    options.title(&title);
                }
                if let Some(alt) = alt.get_untracked() {
                    options.alt(&alt);
                }
                if let Some(interactive) = interactive.get_untracked() {
                    options.interactive(interactive);
                }
                if let Some(z_index_offset) = z_index_offset.get_untracked() {
                    options.z_index_offset(z_index_offset);
                }
                if let Some(opacity) = opacity.get_untracked() {
                    options.opacity(opacity);
                }
                if let Some(rise_on_hover) = rise_on_hover.get_untracked() {
                    options.rise_on_hover(rise_on_hover);
                }
                if let Some(rise_offset) = rise_offset.get_untracked() {
                    options.rise_offset(rise_offset);
                }
                if let Some(pane) = pane.get_untracked() {
                    options.pane(&pane);
                }
                if let Some(shadow_pane) = shadow_pane.get_untracked() {
                    options.shadow_pane(&shadow_pane);
                }
                if let Some(bubbling_mouse_events) = bubbling_mouse_events.get_untracked() {
                    options.bubbling_mouse_events(bubbling_mouse_events);
                }
                if let Some(auto_pan) = auto_pan.get_untracked() {
                    options.auto_pan(auto_pan);
                }
                if let Some((x, y)) = auto_pan_padding.get_untracked() {
                    options.auto_pan_padding(leaflet::Point::new(x, y));
                }
                if let Some(auto_pan_speed) = auto_pan_speed.get_untracked() {
                    options.auto_pan_speed(auto_pan_speed);
                }
                if let Some(icon_url) = icon_url.get_untracked() {
                    let mut icon_options = leaflet::IconOptions::new();
                    icon_options.icon_url(&icon_url);
                    if let Some((x, y)) = icon_size.get_untracked() {
                        icon_options.icon_size(leaflet::Point::new(x, y));
                    }
                    let icon = leaflet::Icon::new(&icon_options);
                    options.icon(icon);
                }
                if let Some(attribution) = attribution.get_untracked() {
                    options.attribution(&attribution);
                }
                let marker =
                    leaflet::Marker::new_with_options(&position.get_untracked().into(), &options);
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

        // children
        //     .map(|children| {
        //         children(cx)
        //             .as_children()
        //             .iter()
        //             .map(|child| child.into_view(cx))
        //             .collect::<Vec<_>>()
        //     })
        //     .unwrap_or_default();
        children.map(|child| child(cx))
    });
    child
}
