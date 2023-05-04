use crate::components::context::LeafletMapContext;
use leaflet::ImageOverlayOptions;
use leptos::*;

#[component(transparent)]
pub fn ImageOverlay(
    cx: Scope,
    #[prop(into)] url: String,
    #[prop(into)] bounds: leaflet::LatLngBounds,
    #[prop(into, optional)] opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] alt: MaybeSignal<Option<String>>,
    #[prop(into, optional)] interactive: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] cross_origin: MaybeSignal<Option<String>>,
    #[prop(into, optional)] cross_origin_toggle: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] error_overlay_url: MaybeSignal<Option<String>>,
    #[prop(into, optional)] z_index: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] class_name: MaybeSignal<Option<String>>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] pane: MaybeSignal<Option<String>>,
    #[prop(into, optional)] attribution: MaybeSignal<Option<String>>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("map context not found");
    create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding image layer: {}", url);
            let mut options = leaflet::ImageOverlayOptions::new();
            if let Some(opacity) = opacity.get_untracked() {
                options.opacity(opacity);
            }
            if let Some(alt) = alt.get_untracked() {
                options.alt(&alt);
            }
            if let Some(interactive) = interactive.get_untracked() {
                options.interactive(interactive);
            }
            if let Some(cross_origin) = cross_origin.get_untracked() {
                options.cross_origin(&cross_origin);
            }
            if let Some(cross_origin_toggle) = cross_origin_toggle.get_untracked() {
                options.cross_origin_toggle(cross_origin_toggle);
            }
            if let Some(error_overlay_url) = error_overlay_url.get_untracked() {
                options.error_overlay_url(&error_overlay_url);
            }
            if let Some(z_index) = z_index.get_untracked() {
                options.z_index(z_index);
            }
            if let Some(class_name) = class_name.get_untracked() {
                options.class_name(&class_name);
            }
            if let Some(bubbling_mouse_events) = bubbling_mouse_events.get_untracked() {
                options.bubbling_mouse_events(bubbling_mouse_events);
            }
            if let Some(pane) = pane.get_untracked() {
                options.pane(&pane);
            }
            if let Some(attribution) = attribution.get_untracked() {
                options.attribution(&attribution);
            }

            let map_layer = leaflet::ImageOverlay::new_with_options(&url, &bounds, &options);
            map_layer.addTo(&map);
            on_cleanup(cx, move || {
                map_layer.remove();
            });
        }
    });
}
