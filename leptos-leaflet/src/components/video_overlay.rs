use crate::components::context::LeafletMapContext;
use leptos::*;
use leptos::logging::log;

#[component(transparent)]
pub fn VideoOverlay(
    #[prop(into)] url: String,
    #[prop(into)] bounds: leaflet::LatLngBounds,
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
    #[prop(into, optional)] autoplay: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] looped: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] keep_aspect_ratio: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] muted: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] plays_inline: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");
    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding image layer: {}", url);
            let mut options = leaflet::VideoOverlayOptions::new();
            if let Some(opacity) = opacity {
                options.opacity(opacity.get_untracked());
            }
            if let Some(alt) = &alt {
                options.alt(&alt.get_untracked());
            }
            if let Some(interactive) = interactive {
                options.interactive(interactive.get_untracked());
            }
            if let Some(cross_origin) = &cross_origin {
                options.cross_origin(&cross_origin.get_untracked());
            }
            if let Some(cross_origin_toggle) = cross_origin_toggle {
                options.cross_origin_toggle(cross_origin_toggle.get_untracked());
            }
            if let Some(error_overlay_url) = &error_overlay_url {
                options.error_overlay_url(&error_overlay_url.get_untracked());
            }
            if let Some(z_index) = z_index {
                options.z_index(z_index.get_untracked());
            }
            if let Some(class_name) = &class_name {
                options.class_name(&class_name.get_untracked());
            }
            if let Some(bubbling_mouse_events) = bubbling_mouse_events {
                options.bubbling_mouse_events(bubbling_mouse_events.get_untracked());
            }
            if let Some(pane) = &pane {
                options.pane(&pane.get_untracked());
            }
            if let Some(attribution) = &attribution {
                options.attribution(&attribution.get_untracked());
            }
            if let Some(autoplay) = autoplay {
                options.autoplay(autoplay.get_untracked());
            }
            if let Some(looped) = looped {
                options.looped(looped.get_untracked());
            }
            if let Some(keep_aspect_ratio) = keep_aspect_ratio {
                options.keep_aspect_ratio(keep_aspect_ratio.get_untracked());
            }
            if let Some(muted) = muted {
                options.muted(muted.get_untracked());
            }
            if let Some(plays_inline) = plays_inline {
                options.plays_inline(plays_inline.get_untracked());
            }

            let map_layer = leaflet::VideoOverlay::new_with_options(&url, &bounds, &options);
            map_layer.addTo(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
