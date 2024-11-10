use leptos::html::Div;
use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::context::{LeafletMapContext, LeafletOverlayContainerContext};
use crate::components::position::Position;

/// A tooltip component.
#[component]
pub fn Tooltip(
    #[prop(into, optional)] position: MaybeSignal<Position>,
    #[prop(into, optional)] permanent: MaybeSignal<bool>,
    #[prop(into, optional, default="auto".into())] direction: MaybeSignal<String>,
    #[prop(into, optional)] sticky: MaybeSignal<bool>,
    #[prop(into, optional, default=0.9.into())] opacity: MaybeSignal<f64>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>();
    let overlay_context = use_context::<LeafletOverlayContainerContext>();

    let content = create_node_ref::<Div>();
    // let content = view! { <div>{children()}</div>};
    create_effect(move |_| {
        let options = leaflet::TooltipOptions::default();
        options.set_permanent(permanent.get_untracked());
        options.set_direction(direction.get_untracked());
        options.set_sticky(sticky.get_untracked());
        options.set_opacity(opacity.get_untracked());

        if let Some(overlay_context) = overlay_context {
            if let Some(layer) = overlay_context.container::<leaflet::Layer>() {
                let tooltip = leaflet::Tooltip::new(&options, Some(layer.unchecked_ref()));
                let content = content.get_untracked().expect("content ref");
                tooltip.set_content(content.unchecked_ref());
                layer.bind_tooltip(&tooltip);
                on_cleanup(move || {
                    tooltip.remove();
                });
            }
        } else if let Some(map) = map_context.expect("Map context not found").map() {
            let tooltip =
                leaflet::Tooltip::new_with_lat_lng(&position.get_untracked().into(), &options);
            let content = content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            tooltip.set_content(html_view);
            tooltip.open_on(&map);
            on_cleanup(move || {
                tooltip.remove();
            });
        }
    });

    view! { <div style="visibility:collapse"><div _ref=content>{children()}</div></div> }
}
