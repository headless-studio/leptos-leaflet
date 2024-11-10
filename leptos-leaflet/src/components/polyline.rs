use crate::components::context::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
};
use crate::components::path_options::{FillRule, LineCap, LineJoin};
use leaflet::{to_lat_lng_array, PolylineOptions};
use leptos::*;

use crate::components::position::Position;
use crate::core::LeafletMaybeSignal;
use crate::{
    setup_layer_leaflet_option, setup_layer_leaflet_option_ref, LayerEvents, MouseEvents,
    PopupEvents, TooltipEvents,
};

/// A polyline overlay that represents a polyline on the map.
#[component(transparent)]
pub fn Polyline(
    #[prop(into)] positions: MaybeSignal<Vec<Position>>,
    #[prop(into, optional)] stroke: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] color: LeafletMaybeSignal<String>,
    #[prop(into, optional)] weight: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] opacity: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] interactive: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] line_cap: LeafletMaybeSignal<LineCap>,
    #[prop(into, optional)] line_join: LeafletMaybeSignal<LineJoin>,
    #[prop(into, optional)] dash_array: LeafletMaybeSignal<String>,
    #[prop(into, optional)] dash_offset: LeafletMaybeSignal<String>,
    #[prop(into, optional)] fill: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] fill_color: LeafletMaybeSignal<String>,
    #[prop(into, optional)] fill_opacity: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] fill_rule: LeafletMaybeSignal<FillRule>,
    #[prop(into, optional)] bubbling_mouse_events: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] class_name: LeafletMaybeSignal<String>,
    #[prop(into, optional)] smooth_factor: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] no_clip: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<leaflet::Polyline>);

    let positions_for_effect = positions.clone();
    let color_clone = color.clone();
    let fill_color_clone = fill_color.clone();
    create_effect(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            let lat_lngs = to_lat_lng_array(&positions.get_untracked());
            let options = PolylineOptions::new();
            setup_layer_leaflet_option!(stroke, options);
            setup_layer_leaflet_option_ref!(color, options);
            setup_layer_leaflet_option!(weight, options);
            setup_layer_leaflet_option!(opacity, options);
            setup_layer_leaflet_option!(interactive, options);
            setup_layer_leaflet_option_ref!(line_cap, options);
            setup_layer_leaflet_option_ref!(line_join, options);
            setup_layer_leaflet_option_ref!(dash_array, options);
            setup_layer_leaflet_option_ref!(dash_offset, options);
            setup_layer_leaflet_option!(fill, options);
            setup_layer_leaflet_option_ref!(fill_color, options);
            setup_layer_leaflet_option!(fill_opacity, options);
            setup_layer_leaflet_option_ref!(fill_rule, options);
            setup_layer_leaflet_option!(bubbling_mouse_events, options);
            setup_layer_leaflet_option_ref!(class_name, options);
            setup_layer_leaflet_option!(smooth_factor, options);
            setup_layer_leaflet_option!(no_clip, options);
            let polyline = leaflet::Polyline::new_with_options(&lat_lngs, &options);

            mouse_events.setup(&polyline);
            layer_events.setup(&polyline);
            popup_events.setup(&polyline);
            tooltip_events.setup(&polyline);

            polyline.add_to(&map);
            update_overlay_context(&polyline);
            overlay.set_value(Some(polyline));
        }
    });

    let position_stop = watch(
        move || positions_for_effect.get(),
        move |pos, _, _| {
            if let Some(polygon) = overlay.get_value() {
                let lat_lngs = to_lat_lng_array(pos);
                polygon.set_lat_lngs(&lat_lngs);
            }
        },
        false,
    );

    let stroke_stop = watch(
        move || stroke.get(),
        move |stroke, _, _| {
            if let (Some(stroke), Some(overlay)) = (stroke, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_stroke(*stroke);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let color_stop = watch(
        move || color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_color(color.to_string());
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let fill_color_stop = watch(
        move || fill_color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_fill_color(color.to_string());
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let opacity_stop = watch(
        move || opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_opacity(*opacity);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let fill_opacity_stop = watch(
        move || fill_opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_fill_opacity(*opacity);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let weight_stop = watch(
        move || weight.get(),
        move |weight, _, _| {
            if let (Some(weight), Some(overlay)) = (weight, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_weight(*weight);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let smooth_factor_stop = watch(
        move || smooth_factor.get(),
        move |smooth_factor, _, _| {
            if let (Some(smooth_factor), Some(overlay)) = (smooth_factor, overlay.get_value()) {
                let options = PolylineOptions::new();
                options.set_smooth_factor(*smooth_factor);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    on_cleanup(move || {
        position_stop();
        stroke_stop();
        color_stop();
        fill_color_stop();
        opacity_stop();
        fill_opacity_stop();
        weight_stop();
        smooth_factor_stop();
        if let Some(overlay) = overlay.get_value() {
            overlay.remove();
        }
    });

    children.map(|child| child())
}
