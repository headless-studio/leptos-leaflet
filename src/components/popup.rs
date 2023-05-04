use crate::components::position::Position;

use leptos::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use crate::components::context::{LeafletMapContext, LeafletOverlayContainerContext};
use crate::MaybeSignalOption;

#[component]
pub fn Popup(
    cx: Scope,
    #[prop(into, optional)] position: MaybeSignal<Position>,
    #[prop(into, optional)] pane: MaybeSignalOption<String>,
    #[prop(into, optional)] offset: MaybeSignalOption<(u32, u32)>,
    #[prop(into, optional)] min_width: MaybeSignalOption<f64>,
    #[prop(into, optional)] max_width: MaybeSignalOption<f64>,
    #[prop(into, optional)] auto_pan: MaybeSignalOption<bool>,
    #[prop(into, optional)] auto_pan_padding_top_left: MaybeSignalOption<(u32, u32)>,
    #[prop(into, optional)] auto_pan_padding_bottom_right: MaybeSignalOption<(u32, u32)>,
    #[prop(into, optional)] auto_pan_padding: MaybeSignalOption<(u32, u32)>,
    #[prop(into, optional)] keep_in_view: MaybeSignalOption<bool>,
    #[prop(into, optional)] close_button: MaybeSignalOption<bool>,
    #[prop(into, optional)] auto_close: MaybeSignalOption<bool>,
    #[prop(into, optional)] close_on_escape_key: MaybeSignalOption<bool>,
    #[prop(into, optional)] close_on_click: MaybeSignalOption<bool>,
    #[prop(into, optional)] class_name: MaybeSignalOption<String>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    let overlay_context = use_context::<LeafletOverlayContainerContext>(cx);

    // Render popup content to a html element
    let content = create_node_ref::<Div>(cx);
    // let content = view! {cx, <div>{children(cx)}</div>};
    create_effect(cx, move |_| {
        log!("Popup context {:?}", map_context);
        let inner_content = content.clone();
        let mut options = leaflet::PopupOptions::default();
        if let Some(pane) = pane.get_untracked() {
            options.pane(&pane);
        }
        if let Some(offset) = offset.get_untracked() {
            options.offset(leaflet::Point::from(offset));
        }
        if let Some(min_width) = min_width.get_untracked() {
            options.min_width(min_width);
        }
        if let Some(max_width) = max_width.get_untracked() {
            options.max_width(max_width);
        }
        if let Some(auto_pan) = auto_pan.get_untracked() {
            options.auto_pan(auto_pan);
        }
        if let Some(auto_pan_padding_top_left) = auto_pan_padding_top_left.get_untracked() {
            options.auto_pan_padding_top_left(leaflet::Point::from(auto_pan_padding_top_left));
        }
        if let Some(auto_pan_padding_bottom_right) = auto_pan_padding_bottom_right.get_untracked() {
            options.auto_pan_padding_bottom_right(leaflet::Point::from(auto_pan_padding_bottom_right));
        }
        if let Some(auto_pan_padding) = auto_pan_padding.get_untracked() {
            options.auto_pan_padding(leaflet::Point::from(auto_pan_padding));
        }
        if let Some(keep_in_view) = keep_in_view.get_untracked() {
            options.keep_in_view(keep_in_view);
        }
        if let Some(close_button) = close_button.get_untracked() {
            options.close_button(close_button);
        }
        if let Some(auto_close) = auto_close.get_untracked() {
            options.auto_close(auto_close);
        }
        if let Some(close_on_escape_key) = close_on_escape_key.get_untracked() {
            options.close_on_escape_key(close_on_escape_key);
        }
        if let Some(close_on_click) = close_on_click.get_untracked() {
            options.close_on_click(close_on_click);
        }
        if let Some(class_name) = class_name.get_untracked() {
            options.class_name(&class_name);
        }
        if let Some(overlay_context) = overlay_context {
            if let (Some(marker), Some(_map)) = (
                overlay_context.container::<leaflet::Layer>(),
                map_context.map(),
            ) {
                log!("Adding popup");
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
            log!("Adding popup");
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
