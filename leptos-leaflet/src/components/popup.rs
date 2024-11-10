use crate::components::position::Position;

use crate::components::context::{LeafletMapContext, LeafletOverlayContainerContext};
use leptos::html::Div;
use leptos::*;
use wasm_bindgen::prelude::*;

/// A popup component for displaying content on the map.
#[component]
pub fn Popup(
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
    let map_context = use_context::<LeafletMapContext>();
    let overlay_context = use_context::<LeafletOverlayContainerContext>();

    // Render popup content to a html element
    let content = create_node_ref::<Div>();

    create_effect(move |_| {
        let inner_content = content;
        let options = leaflet::PopupOptions::default();
        if let Some(pane) = &pane {
            options.set_pane(pane.get_untracked());
        }
        if let Some(offset) = offset {
            options.set_offset(leaflet::Point::from(offset.get_untracked()));
        }
        if let Some(min_width) = min_width {
            options.set_min_width(min_width.get_untracked());
        }
        if let Some(max_width) = max_width {
            options.set_max_width(max_width.get_untracked());
        }
        if let Some(auto_pan) = auto_pan {
            options.set_auto_pan(auto_pan.get_untracked());
        }
        if let Some(auto_pan_padding_top_left) = auto_pan_padding_top_left {
            options.set_auto_pan_padding_top_left(leaflet::Point::from(
                auto_pan_padding_top_left.get_untracked(),
            ));
        }
        if let Some(auto_pan_padding_bottom_right) = auto_pan_padding_bottom_right {
            options.set_auto_pan_padding_bottom_right(leaflet::Point::from(
                auto_pan_padding_bottom_right.get_untracked(),
            ));
        }
        if let Some(auto_pan_padding) = auto_pan_padding {
            options.set_auto_pan_padding(leaflet::Point::from(auto_pan_padding.get_untracked()));
        }
        if let Some(keep_in_view) = keep_in_view {
            options.set_keep_in_view(keep_in_view.get_untracked());
        }
        if let Some(close_button) = close_button {
            options.set_close_button(close_button.get_untracked());
        }
        if let Some(auto_close) = auto_close {
            options.set_auto_close(auto_close.get_untracked());
        }
        if let Some(close_on_escape_key) = close_on_escape_key {
            options.set_close_on_escape_key(close_on_escape_key.get_untracked());
        }
        if let Some(close_on_click) = close_on_click {
            options.set_close_on_click(close_on_click.get_untracked());
        }
        if let Some(class_name) = &class_name {
            options.set_class_name(class_name.get_untracked());
        }
        if let Some(overlay_context) = overlay_context {
            if let Some(marker) = overlay_context.container::<leaflet::Layer>() {
                let popup = leaflet::Popup::new(&options, Some(marker.unchecked_ref()));
                let content = inner_content.get_untracked().expect("content ref");
                let html_view: &JsValue = content.unchecked_ref();
                popup.set_content(html_view);
                marker.bind_popup(&popup);
                on_cleanup(move || {
                    popup.remove();
                });
            }
        } else if let Some(map) = map_context.expect("map context not found").map() {
            let popup =
                leaflet::Popup::new_with_lat_lng(&position.get_untracked().into(), &options);
            let content = inner_content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            popup.set_content(html_view);
            popup.open_on(&map);
            on_cleanup(move || {
                popup.remove();
            });
        }
    });

    view! { <div style="visibility:collapse"><div _ref=content>{children()}</div></div> }
}
