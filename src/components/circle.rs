use crate::components::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
    LeafletOverlayContainerContext, Position,
};
use crate::MaybeSignalString;
use leptos::*;

#[component(transparent)]
pub fn Circle(
    cx: Scope,
    #[prop(into)] center: MaybeSignal<Position>,
    #[prop(into, optional)] stroke: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] color: MaybeSignalString,
    #[prop(into, optional)] weight: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] line_cap: MaybeSignalString,
    #[prop(into, optional)] line_join: MaybeSignalString,
    #[prop(into, optional)] dash_array: MaybeSignalString,
    #[prop(into, optional)] dash_offset: MaybeSignalString,
    #[prop(into, optional)] fill: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] fill_color: MaybeSignalString,
    #[prop(into, optional)] fill_opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] fill_rule: MaybeSignalString,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] class_name: MaybeSignalString,
    #[prop(into)] radius: MaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (child, _) = cx.run_child_scope(|cx| {
        extend_context_with_overlay(cx);

        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let mut options = leaflet::CircleOptions::new();
                options.radius(radius.get_untracked());
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
                let circle =
                    leaflet::Circle::new_with_options(&center.get_untracked().into(), &options);
                circle.addTo(&map);
                update_overlay_context(cx, &circle);
                on_cleanup(cx, move || {
                    circle.remove();
                });
            };
        });

        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let Some(circle) = overlay_context.container::<leaflet::Circle>() {
                circle.setRadius(radius());
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
