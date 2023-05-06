use leaflet::{to_lat_lng_array, PolylineOptions};
use leptos::*;

use crate::components::context::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
    LeafletOverlayContainerContext,
};
use crate::components::path_options::{FillRule, LineCap, LineJoin};
use crate::components::position::Position;

#[component(transparent)]
pub fn Polygon(
    cx: Scope,
    #[prop(into)] positions: MaybeSignal<Vec<Position>>,
    #[prop(into, optional)] stroke: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] color: Option<MaybeSignal<String>>,
    #[prop(into, optional)] weight: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] opacity: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] line_cap: Option<MaybeSignal<LineCap>>,
    #[prop(into, optional)] line_join: Option<MaybeSignal<LineJoin>>,
    #[prop(into, optional)] dash_array: Option<MaybeSignal<String>>,
    #[prop(into, optional)] dash_offset: Option<MaybeSignal<String>>,
    #[prop(into, optional)] fill: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] fill_color: Option<MaybeSignal<String>>,
    #[prop(into, optional)] fill_opacity: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] fill_rule: Option<MaybeSignal<FillRule>>,
    #[prop(into, optional)] bubbling_mouse_events: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] class_name: Option<MaybeSignal<String>>,
    #[prop(into, optional)] smooth_factor: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] no_clip: Option<MaybeSignal<bool>>,
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
                if let Some(stroke) = stroke {
                    options.stroke(stroke.get_untracked());
                }
                if let Some(color) = &color {
                    options.color(&color.get_untracked());
                }
                if let Some(weight) = weight {
                    options.weight(weight.get_untracked());
                }
                if let Some(opacity) = opacity {
                    options.opacity(opacity.get_untracked());
                }
                if let Some(line_cap) = line_cap {
                    options.line_cap(&format!("{}", line_cap.get_untracked()));
                }
                if let Some(line_join) = line_join {
                    options.line_join(&format!("{}", line_join.get_untracked()));
                }
                if let Some(dash_array) = &dash_array {
                    options.dash_array(&dash_array.get_untracked());
                }
                if let Some(dash_offset) = &dash_offset {
                    options.dash_offset(&dash_offset.get_untracked());
                }
                if let Some(fill) = &fill {
                    options.fill(fill.get_untracked());
                }
                if let Some(fill_color) = &fill_color {
                    options.fill_color(&fill_color.get_untracked());
                }
                if let Some(fill_opacity) = fill_opacity {
                    options.fill_opacity(fill_opacity.get_untracked());
                }
                if let Some(fill_rule) = fill_rule {
                    options.fill_rule(&format!("{}", fill_rule.get_untracked()));
                }
                if let Some(bubbling_mouse_events) = bubbling_mouse_events {
                    options.bubbling_mouse_events(bubbling_mouse_events.get_untracked());
                }
                if let Some(class_name) = &class_name {
                    options.class_name(&class_name.get_untracked());
                }
                if let Some(smooth_factor) = smooth_factor {
                    options.smooth_factor(smooth_factor.get_untracked());
                }
                if let Some(no_clip) = &no_clip {
                    options.no_clip(no_clip.get_untracked());
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
