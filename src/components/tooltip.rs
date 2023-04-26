use leptos::*;

use wasm_bindgen::prelude::*;

use super::{LeafletMapContext, LeafletOverlayContainerContext, Position};

#[component]
pub fn Tooltip(
    cx: Scope,
    #[prop(into, optional)] position: MaybeSignal<Position>,
    #[prop(into, optional)] permanent: MaybeSignal<bool>,
    #[prop(into, optional, default="auto".into())] direction: MaybeSignal<String>,
    #[prop(into, optional)] sticky: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    let overlay_context = use_context::<LeafletOverlayContainerContext>(cx);

    let content = view! {cx, <div>{children(cx)}</div>};

    create_effect(cx, move |_| {
        let mut options = leaflet::TooltipOptions::default();
        options.permanent(permanent());
        options.direction(&direction());
        options.sticky(sticky());

        if let Some(overlay_context) = overlay_context.clone() {
            log!("we are in overlay");
            if let (Some(layer), Some(_map)) = (
                overlay_context.container::<leaflet::Layer>(),
                map_context.map(),
            ) {
                log!("Adding tooltip");
                let tooltip = leaflet::Tooltip::new(&options, Some(layer.unchecked_ref()));
                tooltip.setContent(content.unchecked_ref());
                layer.bindPopup(&tooltip);
                on_cleanup(cx, move || {
                    tooltip.remove();
                });
            }
        } else if let Some(map) = map_context.map() {
            log!("Adding tooltip");
            let tooltip = leaflet::Tooltip::new_with_lat_lng(&position().into(), &options);
            let html_view: &JsValue = content.unchecked_ref();
            tooltip.setContent(html_view);
            tooltip.openOn(&map);
            on_cleanup(cx, move || {
                tooltip.remove();
            });
    }
    });
}
