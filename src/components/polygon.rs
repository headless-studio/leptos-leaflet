use leaflet::{to_lat_lng_array, PolylineOptions};
use leptos::*;

use crate::components::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
    LeafletOverlayContainerContext, Position,
};
use crate::MaybeSignalOption;

#[component(transparent)]
pub fn Polygon(
    cx: Scope,
    #[prop(into)] positions: MaybeSignal<Vec<Position>>,
    #[prop(into, optional)] stroke: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] color: MaybeSignalOption<String>,
    #[prop(into, optional)] weight: MaybeSignalOption<f64>,
    #[prop(into, optional)] opacity: MaybeSignalOption<f64>,
    #[prop(into, optional)] line_cap: MaybeSignalOption<String>,
    #[prop(into, optional)] line_join: MaybeSignalOption<String>,
    #[prop(into, optional)] dash_array: MaybeSignalOption<String>,
    #[prop(into, optional)] dash_offset: MaybeSignalOption<String>,
    #[prop(into, optional)] fill: MaybeSignalOption<bool>,
    #[prop(into, optional)] fill_color: MaybeSignalOption<String>,
    #[prop(into, optional)] fill_opacity: MaybeSignalOption<f64>,
    #[prop(into, optional)] fill_rule: MaybeSignalOption<String>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignalOption<bool>,
    #[prop(into, optional)] class_name: MaybeSignalOption<String>,
    #[prop(into, optional)] smooth_factor: MaybeSignalOption<f64>,
    #[prop(into, optional)] no_clip: MaybeSignalOption<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (child, _) = cx.run_child_scope(|cx| {
        extend_context_with_overlay(cx);

        let positions_for_effect = positions.clone();
        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let lat_lngs = to_lat_lng_array(&positions.get_untracked());
                let mut options = PolylineOptions::new();
                if let Some(stroke) = stroke.get_untracked() {
                    options.stroke(stroke);
                }
                if let Some(color) = color.get_untracked() {
                    options.color(&color);
                }
                if let Some(weight) = weight.get_untracked() {
                    options.weight(weight);
                }
                if let Some(opacity) = opacity.get_untracked() {
                    options.opacity(opacity);
                }
                if let Some(line_cap) = line_cap.get_untracked() {
                    options.line_cap(&line_cap);
                }
                if let Some(line_join) = line_join.get_untracked() {
                    options.line_join(&line_join);
                }
                if let Some(dash_array) = dash_array.get_untracked() {
                    options.dash_array(&dash_array);
                }
                if let Some(dash_offset) = dash_offset.get_untracked() {
                    options.dash_offset(&dash_offset);
                }
                if let Some(fill) = fill.get_untracked() {
                    options.fill(fill);
                }
                if let Some(fill_color) = fill_color.get_untracked() {
                    options.fill_color(&fill_color);
                }
                if let Some(fill_opacity) = fill_opacity.get_untracked() {
                    options.fill_opacity(fill_opacity);
                }
                if let Some(fill_rule) = fill_rule.get_untracked() {
                    options.fill_rule(&fill_rule);
                }
                if let Some(bubbling_mouse_events) = bubbling_mouse_events.get_untracked() {
                    options.bubbling_mouse_events(bubbling_mouse_events);
                }
                if let Some(class_name) = class_name.get_untracked() {
                    options.class_name(&class_name);
                }
                if let Some(smooth_factor) = smooth_factor.get_untracked() {
                    options.smooth_factor(smooth_factor);
                }
                if let Some(no_clip) = no_clip.get_untracked() {
                    options.no_clip(no_clip);
                }
                let polygon = leaflet::Polygon::new_with_options(&lat_lngs, &options);
                polygon.addTo(&map);
                update_overlay_context(cx, &polygon);
                on_cleanup(cx, move || {
                    polygon.remove();
                });
            };
        });

        create_effect(cx, move |_| {
            if let Some(polygon) = use_context::<LeafletOverlayContainerContext>(cx)
                .expect("overlay context")
                .container::<leaflet::Polygon>()
            {
                let lat_lngs = to_lat_lng_array(&positions_for_effect());
                polygon.setLatLngs(&lat_lngs);
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
