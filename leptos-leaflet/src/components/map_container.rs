use leptos::{html::Div, *};
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

use leaflet::LocateOptions;

use crate::components::context::provide_leaflet_context;
use crate::components::context::LeafletMapContext;
use crate::components::position::Position;
use crate::{MapEvents, PopupEvents, TooltipEvents};

#[component]
pub fn MapContainer(
    cx: Scope,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] style: MaybeSignal<String>,
    /// Centers the map on the given location
    #[prop(into, optional)]
    center: Option<MaybeSignal<Position>>,
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
    #[prop(optional)] popup_events: PopupEvents,
    #[prop(optional)] tooltip_events: TooltipEvents,
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
            let popup_events = popup_events.clone();
            let tooltip_events = tooltip_events.clone();
            node.on_mount(move |node| {
                let map_context = expect_context::<LeafletMapContext>(cx);
                let node = node.unchecked_ref::<HtmlDivElement>();
                let mut options = leaflet::MapOptions::new();
                options.zoom(zoom());
                if let Some(center) = center {
                    options.center(&center.get().into());
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
                events.setup(&leaflet_map);
                popup_events.setup(&leaflet_map);
                tooltip_events.setup(&leaflet_map);

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
