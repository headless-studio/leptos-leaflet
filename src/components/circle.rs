use crate::components::{
    extend_context_with_overlay, update_overlay_context, LeafletMapContext,
    LeafletOverlayContainerContext, Position,
};
use leaflet::CircleOptions;
use leptos::*;

#[component(transparent)]
pub fn Circle(
    cx: Scope,
    #[prop(into, optional)] options: CircleOptions,
    #[prop(into)] center: MaybeSignal<Position>,
    #[prop(into)] radius: MaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    cx.child_scope(|cx| {
        extend_context_with_overlay(cx);

        create_effect(cx, move |_| {
            if let Some(map) = use_context::<LeafletMapContext>(cx)
                .expect("map context")
                .map()
            {
                log!("Adding circle");
                let mut options = options.clone();
                options.radius(radius.get_untracked());
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

        children
            .map(|children| {
                children(cx)
                    .as_children()
                    .iter()
                    .map(|child| child.into_view(cx))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
    });
}
