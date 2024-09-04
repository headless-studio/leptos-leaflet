use leaflet::{to_lat_lng_array, PolylineOptions};
use leptos::prelude::*;

use super::{extend_context_with_overlay, update_overlay_context, FillRule, LayerEvents, LeafletMapContext, LineCap, LineJoin, MouseEvents, PopupEvents, Position, StringEmptyOption, TooltipEvents
};
use crate::core::{JsMaybeSignal, JsStoredValue, LeafletMaybeSignal};
use crate::{setup_layer_leaflet_option, setup_layer_leaflet_option_ref, setup_layer_leaflet_string};


#[component(transparent)]
pub fn Polyline(
    #[prop(into)] positions: JsMaybeSignal<Vec<Position>>,
    #[prop(into, optional)] stroke: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] color: MaybeSignal<String>,
    #[prop(into, optional)] weight: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] opacity: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] interactive: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] line_cap: LeafletMaybeSignal<LineCap>,
    #[prop(into, optional)] line_join: LeafletMaybeSignal<LineJoin>,
    #[prop(into, optional)] dash_array: MaybeSignal<String>,
    #[prop(into, optional)] dash_offset: LeafletMaybeSignal<String>,
    #[prop(into, optional)] fill: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] fill_color: MaybeSignal<String>,
    #[prop(into, optional)] fill_opacity: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] fill_rule: LeafletMaybeSignal<FillRule>,
    #[prop(into, optional)] bubbling_mouse_events: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] class_name: MaybeSignal<String>,
    #[prop(into, optional)] smooth_factor: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] no_clip: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = JsStoredValue::new(None::<leaflet::Polyline>);

    let positions_for_effect = positions.clone();
    let color_clone = color.clone();
    let fill_color_clone = fill_color.clone();
    Effect::new(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            let lat_lngs = to_lat_lng_array(&positions.get_untracked());
            let options = PolylineOptions::new();
            setup_layer_leaflet_option!(stroke, options);
            setup_layer_leaflet_string!(color, options);
            setup_layer_leaflet_option!(weight, options);
            setup_layer_leaflet_option!(opacity, options);
            setup_layer_leaflet_option!(interactive, options);
            setup_layer_leaflet_option_ref!(line_cap, options);
            setup_layer_leaflet_option_ref!(line_join, options);
            setup_layer_leaflet_string!(dash_array, options);
            setup_layer_leaflet_option_ref!(dash_offset, options);
            setup_layer_leaflet_option!(fill, options);
            setup_layer_leaflet_string!(fill_color, options);
            setup_layer_leaflet_option!(fill_opacity, options);
            setup_layer_leaflet_option_ref!(fill_rule, options);
            setup_layer_leaflet_option!(bubbling_mouse_events, options);
            setup_layer_leaflet_string!(class_name, options);
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

    let position_stop = Effect::watch(
        move || positions_for_effect.get(),
        move |pos, _, _| {
            if let Some(polygon) = overlay.get_value().as_ref() {
                let lat_lngs = to_lat_lng_array(pos);
                polygon.set_lat_lngs(&lat_lngs);
            }
        },
        false,
    );

    let stroke_stop = Effect::watch(
        move || stroke.get(),
        move |stroke, _, _| {
            if let (Some(stroke), Some(overlay)) = (stroke, overlay.get_value().as_ref()) {
                let options = PolylineOptions::new();
                options.set_stroke(*stroke);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let color_stop = Effect::watch(
        move || color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color.to_option(), overlay.get_value().as_ref()) {
                let options = PolylineOptions::new();
                options.set_color(color.to_string());
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let fill_color_stop = Effect::watch(
        move || fill_color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color.to_option(), overlay.get_value().as_ref()) {
                let options = PolylineOptions::new();
                options.set_fill_color(color.to_string());
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let opacity_stop = Effect::watch(
        move || opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value().as_ref()) {
                let options = PolylineOptions::new();
                options.set_opacity(*opacity);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let fill_opacity_stop = Effect::watch(
        move || fill_opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value().as_ref()) {
                let options = PolylineOptions::new();
                options.set_fill_opacity(*opacity);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let weight_stop = Effect::watch(
        move || weight.get(),
        move |weight, _, _| {
            if let (Some(weight), Some(overlay)) = (weight, overlay.get_value().as_ref()) {
                let options = PolylineOptions::new();
                options.set_weight(*weight);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    let smooth_factor_stop = Effect::watch(
        move || smooth_factor.get(),
        move |smooth_factor, _, _| {
            if let (Some(smooth_factor), Some(overlay)) =
                (smooth_factor, overlay.get_value().as_ref())
            {
                let options = PolylineOptions::new();
                options.set_smooth_factor(*smooth_factor);
                overlay.set_style(&options.into())
            }
        },
        false,
    );

    on_cleanup(move || {
        position_stop.stop();
        stroke_stop.stop();
        color_stop.stop();
        fill_color_stop.stop();
        opacity_stop.stop();
        fill_opacity_stop.stop();
        weight_stop.stop();
        smooth_factor_stop.stop();
        if let Some(overlay) = overlay.get_value().as_ref() {
            overlay.remove();
        }
    });

    children.map(|child| child())
}
