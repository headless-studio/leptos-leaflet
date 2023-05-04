use crate::components::position::Position;
use crate::MaybeSignalOption;
use leaflet::CircleOptions;
use leptos::*;
use crate::components::context::{extend_context_with_overlay, LeafletMapContext, LeafletOverlayContainerContext, update_overlay_context};
use crate::components::path_options::{FillRule, LineCap, LineJoin};

#[component(transparent)]
pub fn Circle(
    cx: Scope,
    #[prop(into)] center: MaybeSignal<Position>,
    #[prop(into, optional)] stroke: MaybeSignalOption<bool>,
    #[prop(into, optional)] color: MaybeSignal<String>,
    #[prop(into, optional)] weight: MaybeSignalOption<f64>,
    #[prop(into, optional)] opacity: MaybeSignalOption<f64>,
    #[prop(into, optional)] line_cap: MaybeSignalOption<LineCap>,
    #[prop(into, optional)] line_join: MaybeSignalOption<LineJoin>,
    #[prop(into, optional)] dash_array: MaybeSignal<String>,
    #[prop(into, optional)] dash_offset: MaybeSignal<String>,
    #[prop(into, optional)] fill: MaybeSignalOption<bool>,
    #[prop(into, optional)] fill_color: MaybeSignal<String>,
    #[prop(into, optional)] fill_opacity: MaybeSignalOption<f64>,
    #[prop(into, optional)] fill_rule: MaybeSignalOption<FillRule>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignalOption<bool>,
    #[prop(into, optional)] class_name: MaybeSignal<String>,
    #[prop(into)] radius: MaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (child, _) = cx.run_child_scope(|cx| {
        extend_context_with_overlay(cx);

        let inner_color = color.clone();
        let inner_fill_color = fill_color.clone();
        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let mut options = CircleOptions::new();
                options.radius(radius.get_untracked());
                if let Some(stroke) = stroke.get_untracked() {
                    options.stroke(stroke);
                }
                if !inner_color.get_untracked().is_empty() {
                    options.color(&inner_color.get_untracked());
                }
                if let Some(weight) = weight.get_untracked() {
                    options.weight(weight);
                }
                if let Some(opacity) = opacity.get_untracked() {
                    options.opacity(opacity);
                }
                if let Some(line_cap) = line_cap.get_untracked() {
                    options.line_cap(&format!("{}", line_cap));
                }
                if let Some(line_join) = line_join.get_untracked() {
                    options.line_join(&format!("{}", line_join));
                }
                if !dash_array.get_untracked().is_empty() {
                    options.dash_array(&dash_array.get_untracked());
                }
                if !dash_offset.get_untracked().is_empty() {
                    options.dash_offset(&dash_offset.get_untracked());
                }
                if let Some(fill) = fill.get_untracked() {
                    options.fill(fill);
                }
                if !inner_fill_color.get_untracked().is_empty() {
                    options.fill_color(&inner_fill_color.get_untracked());
                }
                if let Some(fill_opacity) = fill_opacity.get_untracked() {
                    options.fill_opacity(fill_opacity);
                }
                if let Some(fill_rule) = fill_rule.get_untracked() {
                    options.fill_rule(&format!("{}", fill_rule));
                }
                if let Some(bubbling_mouse_events) = bubbling_mouse_events.get_untracked() {
                    options.bubbling_mouse_events(bubbling_mouse_events);
                }
                if !class_name.get_untracked().is_empty() {
                    options.class_name(&class_name.get_untracked());
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

        // Radius
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let Some(circle) = overlay_context.container::<leaflet::Circle>() {
                circle.setRadius(radius());
            }
        });

        // Stroke
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(stroke)) =
                (overlay_context.container::<leaflet::Circle>(), stroke.get())
            {
                let mut options = CircleOptions::new();
                options.stroke(stroke);
                circle.setStyle(&options);
            }
        });

        // Weight
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(weight)) =
                (overlay_context.container::<leaflet::Circle>(), weight.get())
            {
                let mut options = CircleOptions::new();
                options.weight(weight);
                circle.setStyle(&options);
            }
        });

        // Color
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), color) =
                (overlay_context.container::<leaflet::Circle>(), color())
            {
                let mut options = CircleOptions::new();
                options.color(&color);
                circle.setStyle(&options);
            }
        });

        // Fill color
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), color) =
                (overlay_context.container::<leaflet::Circle>(), fill_color())
            {
                let mut options = CircleOptions::new();
                options.fill_color(&color);
                circle.setStyle(&options);
            }
        });

        // Opacity
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(opacity)) = (
                overlay_context.container::<leaflet::Circle>(),
                opacity.get(),
            ) {
                let mut options = CircleOptions::new();
                options.opacity(opacity);
                circle.setStyle(&options);
            }
        });

        // Fill Opacity
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(opacity)) = (
                overlay_context.container::<leaflet::Circle>(),
                fill_opacity.get(),
            ) {
                let mut options = CircleOptions::new();
                options.fill_opacity(opacity);
                circle.setStyle(&options);
            }
        });

        children.map(|child| child(cx))
    });
    child
}
