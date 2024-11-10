use crate::components::bounds::Bounds;
use crate::components::context::LeafletMapContext;
use leptos::logging::log;
use leptos::*;

/// An image overlay component.
#[component(transparent)]
pub fn ImageOverlay(
    #[prop(into)] url: String,
    #[prop(into)] bounds: Bounds,
    #[prop(into, optional)] opacity: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] alt: Option<MaybeSignal<String>>,
    #[prop(into, optional)] interactive: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] cross_origin: Option<MaybeSignal<String>>,
    #[prop(into, optional)] cross_origin_toggle: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] error_overlay_url: Option<MaybeSignal<String>>,
    #[prop(into, optional)] z_index: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] class_name: Option<MaybeSignal<String>>,
    #[prop(into, optional)] bubbling_mouse_events: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] pane: Option<MaybeSignal<String>>,
    #[prop(into, optional)] attribution: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");
    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding image layer: {}", url);
            let options = leaflet::ImageOverlayOptions::new();
            if let Some(opacity) = opacity {
                options.set_opacity(opacity.get_untracked());
            }
            if let Some(alt) = &alt {
                options.set_alt(alt.get_untracked());
            }
            if let Some(interactive) = interactive {
                options.set_interactive(interactive.get_untracked());
            }
            if let Some(cross_origin) = &cross_origin {
                options.set_cross_origin(cross_origin.get_untracked());
            }
            if let Some(cross_origin_toggle) = cross_origin_toggle {
                options.set_cross_origin_toggle(cross_origin_toggle.get_untracked());
            }
            if let Some(error_overlay_url) = &error_overlay_url {
                options.set_error_overlay_url(error_overlay_url.get_untracked());
            }
            if let Some(z_index) = z_index {
                options.set_z_index(z_index.get_untracked());
            }
            if let Some(class_name) = &class_name {
                options.set_class_name(class_name.get_untracked());
            }
            if let Some(bubbling_mouse_events) = bubbling_mouse_events {
                options.set_bubbling_mouse_events(bubbling_mouse_events.get_untracked());
            }
            if let Some(pane) = &pane {
                options.set_pane(pane.get_untracked());
            }
            if let Some(attribution) = &attribution {
                options.set_attribution(attribution.get_untracked());
            }

            let map_layer = leaflet::ImageOverlay::new_with_options(&url, &bounds.into(), &options);
            map_layer.add_to(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
