use crate::components::{provide_leaflet_context, Position};
use leaflet::LocateOptions;
use leptos::{html::Div, *};
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

use super::LeafletMapContext;

#[component]
pub fn MapContainer(
    cx: Scope,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] style: MaybeSignal<String>,
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
    #[prop(optional)] map: Option<WriteSignal<LeafletMap>>,
    /// Inner map child nodes
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let map_ref = create_node_ref::<Div>(cx);

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
                    expect_context::<LeafletMapContext>(cx);
                let node = node.unchecked_ref::<HtmlDivElement>();
                let mut options = leaflet::MapOptions::new();
                options.zoom(zoom());
                if let Some(center) = center.get() {
                    options.center(&center.into());
                }
                log!("Map options: {:?}", options);
                let leaflet_map = leaflet::Map::new(&node.id(), &options);

                // Signal that we have a map
                #[cfg(not(feature = "ssr"))]
                if let Some(set_map) = map {
                    set_map.set(LeafletMap {
                        map: Some(leaflet_map.clone()),
                    });
                }
                if locate() {
                    let mut locate_options = LocateOptions::new();
                    locate_options.enable_high_accuracy(enable_high_accuracy());
                    locate_options.set_view(set_view());
                    locate_options.watch(watch());
                    leaflet_map.locate_with_options(&locate_options);
                    log!("Map locate options: {:?}", locate_options);
                }

                log!("Map node: {:?}", node.id());
                map_context.set_map(&leaflet_map);
            });
        }
    });

    on_cleanup(cx, || {
        #[cfg(not(feature = "ssr"))]
        if let Some(map) = expect_context::<LeafletMapContext>(cx).map_signal().get_untracked() {
            map.remove();
        };
    });

    // Process all children, since this is mostly javascript callbacks we don't "render" the views
    // children
    //     .map(|children| {
    //         children(cx)
    //             .as_children()
    //             .iter()
    //             .map(|child| child.into_view(cx))
    //             .collect::<Vec<_>>()
    //     })
    //     .unwrap_or_default();

    // HydrationCtx::continue_from(next_id);

    // Transition(cx, TransitionProps::builder().fallback(move || view! { cx, <div>{"Loading map..."}</div> }).children(Box::new(move |cx| {
    //     let class = class.clone();
    //     let style = style.clone();
    //     Fragment::lazy(move || vec![{view! {cx, <div class=class _ref=map_ref style=style></div>}}.into_view(cx)])
    // })).build())
    view! {cx, <div class=class _ref=map_ref style=style>{children.map(|child|child(cx))}</div>}
}

#[derive(Debug, Default, Clone)]
pub struct LeafletMap {
    #[cfg(not(feature = "ssr"))]
    pub map: Option<leaflet::Map>,
}

impl LeafletMap {
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "ssr"))]
            map: None,
        }
    }
}
