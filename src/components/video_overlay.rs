use crate::components::LeafletMapContext;
use leaflet::ImageOverlayOptions;
use leptos::*;
use crate::MaybeSignalOption;

#[component(transparent)]
pub fn VideoOverlay(
    cx: Scope,
    #[prop(into)] url: String,
    #[prop(into)] bounds: leaflet::LatLngBounds,
    #[prop(into, optional)] opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] alt: MaybeSignalOption<String>,
    #[prop(into, optional)] interactive: MaybeSignalOption<bool>,
    #[prop(into, optional)] cross_origin: MaybeSignalOption<String>,
    #[prop(into, optional)] cross_origin_toggle: MaybeSignalOption<bool>,
    #[prop(into, optional)] error_overlay_url: MaybeSignalOption<String>,
    #[prop(into, optional)] z_index: MaybeSignalOption<f64>,
    #[prop(into, optional)] class_name: MaybeSignalOption<String>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignalOption<bool>,
    #[prop(into, optional)] pane: MaybeSignalOption<String>,
    #[prop(into, optional)] attribution: MaybeSignalOption<String>,
    #[prop(into, optional)] autoplay: MaybeSignalOption<bool>,
    #[prop(into, optional)] looped: MaybeSignalOption<bool>,
    #[prop(into, optional)] keep_aspect_ratio: MaybeSignalOption<bool>,
    #[prop(into, optional)] muted: MaybeSignalOption<bool>,
    #[prop(into, optional)] plays_inline: MaybeSignalOption<bool>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("map context not found");
    create_effect(cx, move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding image layer: {}", url);
            let mut options = leaflet::VideoOverlayOptions::new();
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
            if let Some(autoplay) = autoplay.get_untracked() {
                options.autoplay(autoplay);
            }
            if let Some(looped) = looped.get_untracked() {
                options.looped(looped);
            }
            if let Some(keep_aspect_ratio) = keep_aspect_ratio.get_untracked() {
                options.keep_aspect_ratio(keep_aspect_ratio);
            }
            if let Some(muted) = muted.get_untracked() {
                options.muted(muted);
            }
            if let Some(plays_inline) = plays_inline.get_untracked() {
                options.plays_inline(plays_inline);
            }

            let map_layer = leaflet::VideoOverlay::new_with_options(&url, &bounds, &options);
            map_layer.addTo(&map);
            on_cleanup(cx, move || {
                map_layer.remove();
            });
        }
    });
}
