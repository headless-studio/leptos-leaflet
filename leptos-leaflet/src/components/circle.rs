use crate::components::context::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
    LeafletOverlayContainerContext,
};
use crate::components::path_options::{FillRule, LineCap, LineJoin};
use crate::components::position::Position;
use leaflet::CircleOptions;
use leptos::*;

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
    #[prop(into)] radius: MaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (child, _) = cx.run_child_scope(|cx| {
        extend_context_with_overlay(cx);

        let start_color = color.clone();
        let start_fill_color = fill_color.clone();
        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                let mut options = CircleOptions::new();
                options.radius(radius.get_untracked());
                if let Some(stroke) = stroke {
                    options.stroke(stroke.get_untracked());
                }
                if let Some(color) = &start_color {
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
                if let Some(fill) = fill {
                    options.fill(fill.get_untracked());
                }
                if let Some(fill_color) = &start_fill_color {
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
                (overlay_context.container::<leaflet::Circle>(), &stroke)
            {
                let mut options = CircleOptions::new();
                options.stroke(stroke.get());
                circle.setStyle(&options);
            }
        });

        // Weight
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(weight)) =
                (overlay_context.container::<leaflet::Circle>(), &weight)
            {
                let mut options = CircleOptions::new();
                options.weight(weight.get());
                circle.setStyle(&options);
            }
        });

        // Color
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(color)) =
                (overlay_context.container::<leaflet::Circle>(), &color)
            {
                let mut options = CircleOptions::new();
                options.color(&color.get());
                circle.setStyle(&options);
            }
        });

        // Fill color
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(color)) =
                (overlay_context.container::<leaflet::Circle>(), &fill_color)
            {
                let mut options = CircleOptions::new();
                options.fill_color(&color.get());
                circle.setStyle(&options);
            }
        });

        // Opacity
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(opacity)) =
                (overlay_context.container::<leaflet::Circle>(), &opacity)
            {
                let mut options = CircleOptions::new();
                options.opacity(opacity.get());
                circle.setStyle(&options);
            }
        });

        // Fill Opacity
        create_effect(cx, move |_| {
            let overlay_context =
                use_context::<LeafletOverlayContainerContext>(cx).expect("overlay context");
            if let (Some(circle), Some(fill_opacity)) = (
                overlay_context.container::<leaflet::Circle>(),
                &fill_opacity,
            ) {
                let mut options = CircleOptions::new();
                options.fill_opacity(fill_opacity.get());
                circle.setStyle(&options);
            }
        });

        children.map(|child| child(cx))
    });
    child
}
