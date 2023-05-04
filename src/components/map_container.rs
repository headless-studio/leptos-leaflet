use crate::components::Position;
use leaflet::LocateOptions;
use leptos::{*, html::Div};
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;
use crate::components::context::provide_leaflet_context;
use crate::MapEvents;

use crate::components::context::LeafletMapContext;

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
    #[prop(optional)] events: MapEvents,
    /// Inner map child nodes
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let map_ref = create_node_ref::<Div>(cx);

    provide_leaflet_context(cx);

    create_effect(cx, move |_| {
        if let Some(node) = map_ref() {
            let center = center;
            let html_node = node.unchecked_ref::<HtmlDivElement>();
            // Randomize the id of the map
            if html_node.id().is_empty() {
                let id = format!("map-{}", rand::random::<u64>());
                node.clone().id(id);
            }
            let events = events.clone();
            node.on_mount(move |node| {
                let map_context = expect_context::<LeafletMapContext>(cx);
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

                // Setup events
                let events_clone = events.clone();
                if let Some(location_found) = events_clone.take_location_found() {
                    leaflet_map.on_location_found(location_found);
                }
                let events_clone = events.clone();
                if let Some(location_error) = events_clone.take_location_error() {
                    leaflet_map.on_location_error(location_error);
                }
                let events_clone = events.clone();
                if let Some(popup_open) = events_clone.take_popup_open() {
                    leaflet_map.on_popup_open(popup_open);
                }
                let events_clone = events.clone();
                if let Some(popup_close) = events_clone.take_popup_close() {
                    leaflet_map.on_popup_close(popup_close);
                }
                let events_clone = events.clone();
                if let Some(load) = events_clone.take_load() {
                    leaflet_map.on_load(load);
                }
                let events_clone = events.clone();
                if let Some(unload) = events_clone.take_unload() {
                    leaflet_map.on_unload(unload);
                }
                let events_clone = events.clone();
                if let Some(resize) = events_clone.take_resize() {
                    leaflet_map.on_resize(resize);
                }

                if locate() {
                    let mut locate_options = LocateOptions::new();
                    locate_options.enable_high_accuracy(enable_high_accuracy());
                    locate_options.set_view(set_view());
                    locate_options.watch(watch());
                    leaflet_map.locateWithOptions(&locate_options);

                    // leaflet_map.on("locationfound", );
                    log!("Map locate options: {:?}", locate_options);
                }

                log!("Map node: {:?}", node.id());
                map_context.set_map(&leaflet_map);
            });
        }
    });

    on_cleanup(cx, move || {
        #[cfg(not(feature = "ssr"))]
        if let Some(map) = expect_context::<LeafletMapContext>(cx)
            .map_signal()
            .get_untracked()
        {
            map.remove();
        };
    });

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
