use crate::components::{provide_leaflet_context, Position};
use leaflet::LocateOptions;
use leptos::{html::Div, leptos_dom::HydrationCtx, *};
use leptos::leptos_dom::is_server;
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

use super::LeafletMapContext;

#[component]
pub fn MapContainer(
    cx: Scope,
    #[prop(into, optional)]
    class: MaybeSignal<String>,
    #[prop(into, optional)]
    style: MaybeSignal<String>,
    /// Centers the map on the given location
    #[prop(into, optional)]
    center: MaybeSignal<Option<Position>>,
    /// Zoom level of the map. Defaults to 10.0
    #[prop(into, optional, default = 10.0.into())]
    zoom: MaybeSignal<f64>,
    /// Use geolocation from the browser to track the user
    #[prop(into, optional)]
    locate: MaybeSignal<bool>,
    /// Tracks position of the user on the map
    #[prop(into, optional)]
    watch: MaybeSignal<bool>,
    /// Enables high-accuracy tracking
    #[prop(into, optional)]
    enable_high_accuracy: MaybeSignal<bool>,
    /// Sets the view of the map if geolocation is available
    #[prop(into, optional)]
    set_view: MaybeSignal<bool>,
    // on_locate_found: Box<dyn FnOnce()>,
    /// Inner map child nodes
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let map_ref = create_node_ref::<Div>(cx);

    // let next_id = HydrationCtx::next_component();
    provide_leaflet_context(cx);

    create_effect(cx, move |_| {
        if let Some(node) = map_ref() {
            let center = center.clone();
            let html_node = node.unchecked_ref::<HtmlDivElement>();
            // Randomize the id of the map
            if html_node.id().is_empty() {
                let id = format!("map-{}", rand::random::<u64>());
                node.clone().id(id);
            }
            node.on_mount(move |node| {
                let map_context =
                    use_context::<LeafletMapContext>(cx).expect("Map context not found");
                let node = node.unchecked_ref::<HtmlDivElement>();
                let mut options = leaflet::MapOptions::new();
                options.zoom(zoom());
                if let Some(center) = center.get() {
                    options.center(&center.into());
                }
                log!("Map options: {:?}", options);
                let map = leaflet::Map::new(&node.id(), &options);
                if locate() {
                    let mut locate_options = LocateOptions::new();
                    locate_options.enable_high_accuracy(enable_high_accuracy());
                    locate_options.set_view(set_view());
                    locate_options.watch(watch());
                    map.locate_with_options(&locate_options);
                    log!("Map locate options: {:?}", locate_options);
                }

                log!("Map node: {:?}", node.id());
                map_context.set_map(map);
            });
        }
    });

    // Process all children, since this is mostly javascript callbacks we don't "render" the views
    children
        .map(|children| {
            children(cx)
                .as_children()
                .iter()
                .map(|child| child.into_view(cx))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    // HydrationCtx::continue_from(next_id);
    
    // Transition(cx, TransitionProps::builder().fallback(move || view! { cx, <div>{"Loading map..."}</div> }).children(Box::new(move |cx| {
    //     let class = class.clone();
    //     let style = style.clone();
    //     Fragment::lazy(move || vec![{view! {cx, <div class=class _ref=map_ref style=style></div>}}.into_view(cx)])
    // })).build())
    view! {cx, <div class=class _ref=map_ref style=style></div>}
}

fn handle_map_events(cx: Scope) {}