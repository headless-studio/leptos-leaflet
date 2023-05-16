use leptos::*;

use leaflet::{PolylineOptions, to_lat_lng_array};

use crate::{effect_update_on_change, effect_update_on_change_ref, LayerEvents, MouseEvents, PopupEvents, setup_layer_option, setup_layer_option_ref, setup_layer_option_str, TooltipEvents};
use crate::components::context::{
    extend_context_with_overlay, LeafletMapContext, LeafletOverlayContainerContext,
    update_overlay_context,
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
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (child, _) = cx.run_child_scope(|cx| {
        extend_context_with_overlay(cx);

        let positions_for_effect = positions.clone();
        let color_clone = color.clone();
        let fill_color_clone = fill_color.clone();
        // This effect just setups the polygon when we get a map
        create_effect(cx, move |done| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let lat_lngs = to_lat_lng_array(&positions.get_untracked());
                let mut options = PolylineOptions::new();
                setup_layer_option!(stroke, options);
                setup_layer_option_ref!(color, options);
                setup_layer_option!(weight, options);
                setup_layer_option!(opacity, options);
                setup_layer_option_str!(line_cap, options);
                setup_layer_option_str!(line_join, options);
                setup_layer_option_ref!(dash_array, options);
                setup_layer_option_ref!(dash_offset, options);
                setup_layer_option!(fill, options);
                setup_layer_option_ref!(fill_color, options);
                setup_layer_option!(fill_opacity, options);
                setup_layer_option_str!(fill_rule, options);
                setup_layer_option!(bubbling_mouse_events, options);
                setup_layer_option_str!(class_name, options);
                setup_layer_option!(smooth_factor, options);
                setup_layer_option!(no_clip, options);

                let polygon = leaflet::Polygon::new_with_options(&lat_lngs, &options);

                mouse_events.setup(&polygon);
                layer_events.setup(&polygon);
                popup_events.setup(&polygon);
                tooltip_events.setup(&polygon);

                polygon.addTo(&map);
                update_overlay_context(cx, &polygon);
                on_cleanup(cx, move || {
                    polygon.remove();
                });
                true
            } else {
                false
            }
        });

        create_effect(cx, move |_| {
            positions_for_effect.track();
            if let Some(polygon) = use_context::<LeafletOverlayContainerContext>(cx)
                .expect("overlay context")
                .container::<leaflet::Polygon>()
            {
                let lat_lngs = to_lat_lng_array(&positions_for_effect());
                polygon.setLatLngs(&lat_lngs);
            }
        });

        effect_update_on_change!(cx, leaflet::Polygon, leaflet::PolylineOptions, stroke);
        effect_update_on_change_ref!(cx, leaflet::Polygon, leaflet::PolylineOptions, color, color_clone);
        effect_update_on_change_ref!(cx, leaflet::Polygon, leaflet::PolylineOptions, fill_color, fill_color_clone);
        effect_update_on_change!(cx, leaflet::Polygon, leaflet::PolylineOptions, opacity);
        effect_update_on_change!(cx, leaflet::Polygon, leaflet::PolylineOptions, fill_opacity);
        effect_update_on_change!(cx, leaflet::Polygon, leaflet::PolylineOptions, weight);
        effect_update_on_change!(cx, leaflet::Polygon, leaflet::PolylineOptions, smooth_factor);

        children.map(|child| child(cx))
    });
    child
}
