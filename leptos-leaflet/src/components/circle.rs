use crate::components::context::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
    LeafletOverlayContainerContext,
};
use crate::components::path_options::{FillRule, LineCap, LineJoin};
use crate::components::position::Position;
use leaflet::CircleOptions;
use leptos::*;
use crate::{effect_update_on_change, effect_update_on_change_ref, LayerEvents, MouseEvents, PopupEvents, setup_layer_option, setup_layer_option_ref, setup_layer_option_str, TooltipEvents};

#[component(transparent)]
pub fn Circle(
    cx: Scope,
    #[prop(into)] center: MaybeSignal<Position>,
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
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,

    #[prop(into)] radius: MaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (child, _) = cx.run_child_scope(|cx| {
        extend_context_with_overlay(cx);

        let color_clone = color.clone();
        let fill_color_clone = fill_color.clone();
        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let mut options = CircleOptions::new();
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
                let circle =
                    leaflet::Circle::new_with_options(&center.get_untracked().into(), &options);

                mouse_events.setup(&circle);
                popup_events.setup(&circle);
                tooltip_events.setup(&circle);
                layer_events.setup(&circle);

                circle.addTo(&map);
                update_overlay_context(cx, &circle);
                on_cleanup(cx, move || {
                    circle.remove();
                });
            };
        });

        // Radius
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let Some(circle) = overlay_context.container::<leaflet::Circle>() {
                circle.setRadius(radius());
            }
        });

        effect_update_on_change!(cx, leaflet::Circle, leaflet::CircleOptions, stroke);
        effect_update_on_change!(cx, leaflet::Circle, leaflet::CircleOptions, weight);
        effect_update_on_change_ref!(cx, leaflet::Circle, leaflet::CircleOptions, color, color_clone);
        effect_update_on_change_ref!(cx, leaflet::Circle, leaflet::CircleOptions, fill_color, fill_color_clone);
        effect_update_on_change!(cx, leaflet::Circle, leaflet::CircleOptions, opacity);
        effect_update_on_change!(cx, leaflet::Circle, leaflet::CircleOptions, fill_opacity);

        children.map(|child| child(cx))
    });
    child
}
