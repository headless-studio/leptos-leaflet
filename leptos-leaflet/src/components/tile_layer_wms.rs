use leptos::logging::warn;
use leptos::prelude::*;
use leaflet::{Map, TileLayerWms as LeafletTileLayerWms, TileLayerWmsOptions};

use super::{LeafletMapContext, MapEvents, TileLayerWmsContext};
use crate::core::IntoThreadSafeJsValue;

/// A WMS tile layer component.
#[component(transparent)]
pub fn TileLayerWms(
    #[prop(into)] url: String,
    options: StoredValue<TileLayerWmsOptions, LocalStorage>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] bring_to_front: bool,
    #[prop(optional)] bring_to_back: bool,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");
    let wms_context = TileLayerWmsContext::new();
    provide_context(wms_context);

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            let options = options.get_value();
            let map_layer =
                leaflet::TileLayerWms::new_options(&url, &options).into_thread_safe_js_value();
            map_layer.add_to(&map);
            wms_context.set_wms(&map_layer);

            match (bring_to_front, bring_to_back) {
                (true, true) => warn!("The parameters are set to bring the layer to front and back at the same time. Ignoring these parameters..."),
                (true, false) => {map_layer.bring_to_front();}
                (false, true) => {map_layer.bring_to_back();}
                (false, false) => (),
            }

            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
    children.map_or(view! { <>""</> }.into_any(), |c| {
        view! { <>{ c() }</>}.into_any()
    })
}

#[component(transparent)]
pub fn TileLayerWmsEvents<F>(map_events: F) -> impl IntoView
where
    F: Fn(&Map, &LeafletTileLayerWms) -> MapEvents + 'static,
{
    let map_ctx =
        use_context::<LeafletMapContext>().expect("LeafletMapContext not available.");
    let wms_ctx =
        use_context::<TileLayerWmsContext>().expect("TileLayerWmsContext not available.");

    Effect::new(move |_| {
        let map = map_ctx.map();
        let wms = wms_ctx.wms();
        if let (Some(m), Some(w)) = (map, wms) {
            let events = map_events(&m, &w);
            events.setup(&m);
        }
    });
}
