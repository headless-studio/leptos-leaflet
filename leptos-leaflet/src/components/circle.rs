use leaflet::CircleOptions;
use leptos::prelude::*;

use super::{
    extend_context_with_overlay, FillRule, LayerEvents, LeafletMapContext, LineCap, LineJoin,
    MouseEvents, MoveEvents, PopupEvents, Position, StringEmptyOption, TooltipEvents,
};
use crate::{
    core::{JsMaybeSignal, JsStoredValue},
    setup_layer_leaflet_option, setup_layer_leaflet_option_ref, setup_layer_leaflet_string,
};

#[component(transparent)]
pub fn Circle(
    #[prop(into)] center: JsMaybeSignal<Position>,
    #[prop(into, optional)] stroke: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] color: MaybeSignal<String>,
    #[prop(into, optional)] weight: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] interactive: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] line_cap: MaybeSignal<Option<LineCap>>,
    #[prop(into, optional)] line_join: MaybeSignal<Option<LineJoin>>,
    #[prop(into, optional)] dash_array: MaybeSignal<String>,
    #[prop(into, optional)] dash_offset: MaybeSignal<String>,
    #[prop(into, optional)] fill: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] fill_color: MaybeSignal<String>,
    #[prop(into, optional)] fill_opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] fill_rule: MaybeSignal<Option<FillRule>>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] class_name: MaybeSignal<String>,
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,
    #[prop(into, optional)] move_events: MoveEvents,

    #[prop(into)] radius: MaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = center;
    let overlay_context = extend_context_with_overlay();
    let overlay = JsStoredValue::new_local(None::<leaflet::Circle>);

    let color_clone = color.clone();
    let fill_color_clone = fill_color.clone();
    Effect::new(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            let options = CircleOptions::new();
            setup_layer_leaflet_option!(stroke, options);
            setup_layer_leaflet_string!(color, options);
            setup_layer_leaflet_option!(weight, options);
            setup_layer_leaflet_option!(opacity, options);
            setup_layer_leaflet_option!(interactive, options);
            setup_layer_leaflet_option_ref!(line_cap, options);
            setup_layer_leaflet_option_ref!(line_join, options);
            setup_layer_leaflet_string!(dash_array, options);
            setup_layer_leaflet_string!(dash_offset, options);
            setup_layer_leaflet_option!(fill, options);
            setup_layer_leaflet_string!(fill_color, options);
            setup_layer_leaflet_option!(fill_opacity, options);
            setup_layer_leaflet_option_ref!(fill_rule, options);
            setup_layer_leaflet_option!(bubbling_mouse_events, options);
            setup_layer_leaflet_string!(class_name, options);
            let circle =
                leaflet::Circle::new_with_options(&center.get_untracked().into(), &options);

            leaflet::Circle::set_radius(&circle, radius.get_untracked());

            mouse_events.setup(&circle);
            popup_events.setup(&circle);
            tooltip_events.setup(&circle);
            layer_events.setup(&circle);
            move_events.setup(&circle);

            circle.add_to(&map);
            overlay_context.set_container(&circle);
            overlay.set_value(Some(circle));
        };
    });

    let radius_stop = Effect::watch(
        move || radius.get(),
        move |radius, _, _| {
            if let Some(polygon) = overlay.get_value().as_ref() {
                polygon.set_radius(*radius);
            }
        },
        false,
    );

    let stroke_stop = Effect::watch(
        move || stroke.get(),
        move |stroke, _, _| {
            if let (Some(stroke), Some(overlay)) = (stroke, overlay.get_value().as_ref()) {
                let options = CircleOptions::new();
                options.set_stroke(*stroke);
                overlay.set_style(&options);
            }
        },
        false,
    );

    let color_stop = Effect::watch(
        move || color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color.to_option(), overlay.get_value().as_ref())
            {
                let options = CircleOptions::new();
                options.set_color(color.to_string());
                overlay.set_style(&options);
            }
        },
        false,
    );

    let fill_color_stop = Effect::watch(
        move || fill_color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color.to_option(), overlay.get_value().as_ref())
            {
                let options = CircleOptions::new();
                options.set_fill_color(color.to_string());
                overlay.set_style(&options);
            }
        },
        false,
    );

    let opacity_stop = Effect::watch(
        move || opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value().as_ref()) {
                let options = CircleOptions::new();
                options.set_opacity(*opacity);
                overlay.set_style(&options);
            }
        },
        false,
    );

    let fill_opacity_stop = Effect::watch(
        move || fill_opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value().as_ref()) {
                let options = CircleOptions::new();
                options.set_fill_opacity(*opacity);
                overlay.set_style(&options);
            }
        },
        false,
    );

    let weight_stop = Effect::watch(
        move || weight.get(),
        move |weight, _, _| {
            if let (Some(weight), Some(overlay)) = (weight, overlay.get_value().as_ref()) {
                let options = CircleOptions::new();
                options.set_weight(*weight);
                overlay.set_style(&options);
            }
        },
        false,
    );

    let position_stop = Effect::watch(
        move || position_tracking.get(),
        move |position_tracking, _, _| {
            if let Some(circle) = overlay.get_value().as_ref() {
                circle.set_lat_lng(&position_tracking.as_lat_lng());
            }
        },
        false,
    );

    on_cleanup(move || {
        position_stop.stop();
        radius_stop.stop();
        stroke_stop.stop();
        color_stop.stop();
        fill_color_stop.stop();
        opacity_stop.stop();
        fill_opacity_stop.stop();
        weight_stop.stop();
        if let Some(overlay) = overlay.try_get_value().flatten().as_ref() {
            overlay.remove();
        }
    });

    children.map(|child| child())
}
