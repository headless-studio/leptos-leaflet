use leptos::prelude::*;

use leaflet::{to_lat_lng_array, PolylineOptions};

use super::{
    extend_context_with_overlay, update_overlay_context, use_pane_context, FillRule, LayerEvents,
    LeafletMapContext, LineCap, LineJoin, MouseEvents, PaneRendererScope, PopupEvents, Position,
    StringEmptyOption, TooltipEvents,
};
use crate::core::JsStoredValue;
use crate::{
    setup_layer_leaflet_option, setup_layer_leaflet_option_ref, setup_layer_leaflet_string,
};
use tracing::debug;

/// A polygon overlay that represents a polygon on the map.
#[component(transparent)]
pub fn Polygon(
    #[prop(into)] positions: Signal<Vec<Position>>,
    #[prop(into, optional)] stroke: Signal<Option<bool>>,
    #[prop(into, optional)] color: Signal<String>,
    #[prop(into, optional)] weight: Signal<Option<f64>>,
    #[prop(into, optional)] opacity: Signal<Option<f64>>,
    #[prop(into, optional)] interactive: Signal<Option<bool>>,
    #[prop(into, optional)] line_cap: Signal<Option<LineCap>>,
    #[prop(into, optional)] line_join: Signal<Option<LineJoin>>,
    #[prop(into, optional)] dash_array: Signal<String>,
    #[prop(into, optional)] dash_offset: Signal<String>,
    #[prop(into, optional)] fill: Signal<Option<bool>>,
    #[prop(into, optional)] fill_color: Signal<String>,
    #[prop(into, optional)] fill_opacity: Signal<Option<f64>>,
    #[prop(into, optional)] fill_rule: Signal<Option<FillRule>>,
    #[prop(into, optional)] bubbling_mouse_events: Signal<Option<bool>>,
    #[prop(into, optional)] class_name: Signal<String>,
    #[prop(into, optional)] smooth_factor: Signal<Option<f64>>,
    #[prop(into, optional)] no_clip: Signal<Option<bool>>,
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = JsStoredValue::new_local(None::<leaflet::Polygon>);

    let positions_for_effect = positions;
    let color_clone = color;
    let fill_color_clone = fill_color;
    // This effect just setups the polygon when we get a map
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
            setup_layer_leaflet_string!(dash_offset, options);
            setup_layer_leaflet_option!(fill, options);
            setup_layer_leaflet_string!(fill_color, options);
            setup_layer_leaflet_option!(fill_opacity, options);
            setup_layer_leaflet_option_ref!(fill_rule, options);
            setup_layer_leaflet_option!(bubbling_mouse_events, options);
            setup_layer_leaflet_string!(class_name, options);

            // Set pane and renderer if available from pane context
            if let Some(pane_context) = use_pane_context() {
                debug!("Polygon using pane: {}", pane_context.name());
                options.set_pane(pane_context.name().to_string());

                match pane_context.renderer_scope() {
                    PaneRendererScope::PaneSpecificSvg => {
                        debug!(
                            "Setting pane-specific SVG renderer for pane: {}",
                            pane_context.name()
                        );
                        if let Some(renderer) = pane_context.svg_renderer() {
                            options.set_renderer(renderer.clone().into());
                        }
                    }
                    PaneRendererScope::PaneSpecificCanvas => {
                        debug!(
                            "Setting pane-specific Canvas renderer for pane: {}",
                            pane_context.name()
                        );
                        if let Some(renderer) = pane_context.canvas_renderer() {
                            options.set_renderer(renderer.clone().into());
                        }
                    }
                    PaneRendererScope::Global => {
                        debug!("Using global renderer for pane: {}", pane_context.name());
                        // Use global rendering but still set the pane
                        options.set_pane(pane_context.name().to_string());
                    }
                }
            } else {
                debug!("Pane context NOT available.");
            }

            let polygon = leaflet::Polygon::new_with_options(&lat_lngs, &options);

            mouse_events.setup(&polygon);
            layer_events.setup(&polygon);
            popup_events.setup(&polygon);
            tooltip_events.setup(&polygon);

            polygon.add_to(&map);
            update_overlay_context(&polygon);
            overlay.set_value(Some(polygon));
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
            if let (Some(color), Some(overlay)) = (color.to_option(), overlay.get_value().as_ref())
            {
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
            if let (Some(color), Some(overlay)) = (color.to_option(), overlay.get_value().as_ref())
            {
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
        if let Some(overlay) = overlay.try_get_value().flatten().as_ref() {
            overlay.remove();
        }
    });

    children.map(|child| child())
}
