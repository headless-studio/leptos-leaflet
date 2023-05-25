use crate::components::position::Position;

use crate::components::context::{LeafletMapContext, LeafletOverlayContainerContext};
use leptos::html::Div;
use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Popup(
    cx: Scope,
    #[prop(into, optional)] position: MaybeSignal<Position>,
    #[prop(into, optional)] pane: Option<MaybeSignal<String>>,
    #[prop(into, optional)] offset: Option<MaybeSignal<(u32, u32)>>,
    #[prop(into, optional)] min_width: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] max_width: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] auto_pan: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] auto_pan_padding_top_left: Option<MaybeSignal<(u32, u32)>>,
    #[prop(into, optional)] auto_pan_padding_bottom_right: Option<MaybeSignal<(u32, u32)>>,
    #[prop(into, optional)] auto_pan_padding: Option<MaybeSignal<(u32, u32)>>,
    #[prop(into, optional)] keep_in_view: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] close_button: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] auto_close: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] close_on_escape_key: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] close_on_click: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] class_name: Option<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    let overlay_context = use_context::<LeafletOverlayContainerContext>(cx);

    // Render popup content to a html element
    let content = create_node_ref::<Div>(cx);
    // let content = view! {cx, <div>{children(cx)}</div>};
    create_effect(cx, move |_| {
        let inner_content = content.clone();
        let mut options = leaflet::PopupOptions::default();
        if let Some(pane) = &pane {
            options.pane(&pane.get_untracked());
        }
        if let Some(offset) = offset {
            options.offset(leaflet::Point::from(offset.get_untracked()));
        }
        if let Some(min_width) = min_width {
            options.min_width(min_width.get_untracked());
        }
        if let Some(max_width) = max_width {
            options.max_width(max_width.get_untracked());
        }
        if let Some(auto_pan) = auto_pan {
            options.auto_pan(auto_pan.get_untracked());
        }
        if let Some(auto_pan_padding_top_left) = auto_pan_padding_top_left {
            options.auto_pan_padding_top_left(leaflet::Point::from(
                auto_pan_padding_top_left.get_untracked(),
            ));
        }
        if let Some(auto_pan_padding_bottom_right) = auto_pan_padding_bottom_right {
            options.auto_pan_padding_bottom_right(leaflet::Point::from(
                auto_pan_padding_bottom_right.get_untracked(),
            ));
        }
        if let Some(auto_pan_padding) = auto_pan_padding {
            options.auto_pan_padding(leaflet::Point::from(auto_pan_padding.get_untracked()));
        }
        if let Some(keep_in_view) = keep_in_view {
            options.keep_in_view(keep_in_view.get_untracked());
        }
        if let Some(close_button) = close_button {
            options.close_button(close_button.get_untracked());
        }
        if let Some(auto_close) = auto_close {
            options.auto_close(auto_close.get_untracked());
        }
        if let Some(close_on_escape_key) = close_on_escape_key {
            options.close_on_escape_key(close_on_escape_key.get_untracked());
        }
        if let Some(close_on_click) = close_on_click {
            options.close_on_click(close_on_click.get_untracked());
        }
        if let Some(class_name) = &class_name {
            options.class_name(&class_name.get_untracked());
        }
        if let Some(overlay_context) = overlay_context {
            if let (Some(marker), Some(_map)) = (
                overlay_context.container::<leaflet::Layer>(),
                map_context.map(),
            ) {
                let popup = leaflet::Popup::new(&options, Some(marker.unchecked_ref()));
                let content = inner_content.get_untracked().expect("content ref");
                let html_view: &JsValue = content.unchecked_ref();
                popup.setContent(html_view);
                marker.bindPopup(&popup);
                on_cleanup(cx, move || {
                    popup.remove();
                });
            }
        } else if let Some(map) = map_context.map() {
            let popup = leaflet::Popup::newWithLatLng(&position().into(), &options);
            let content = inner_content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            popup.setContent(html_view);
            popup.openOn(&map);
            on_cleanup(cx, move || {
                popup.remove();
            });
        }
    });

    view! {cx, <div style="visibility:collapse"><div _ref=content>{children(cx)}</div></div> }
}
