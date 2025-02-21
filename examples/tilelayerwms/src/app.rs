use geojson::GeoJson;
use leptos::{prelude::*, task::spawn_local};
use leptos_leaflet::leaflet::{TileLayerWmsOptions, WmsRequestBuilder};
use leptos_leaflet::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (pre_text, set_pre_text) =
        RwSignal::new("Please click into the map to start a request.".to_string()).split();

    let options = TileLayerWmsOptions::new();
    options.set_layers("OSM-WMS".to_string());

    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let abort_signal = StoredValue::new_local(abort_signal);
    let abort_controller = StoredValue::new_local(abort_controller);
    on_cleanup(move || {
        if let Some(abort_controller) = abort_controller.try_get_value().flatten() {
            abort_controller.abort()
        }
    });

    let map_events = move |map: &leptos_leaflet::leaflet::Map,
                           wms: &leptos_leaflet::leaflet::TileLayerWms| {
        let mut events = MapEvents::new();
        let map_clone = map.clone();
        let wms_clone = wms.clone();
        events = events.mouse_click(move |m| {
            let url = WmsRequestBuilder::default()
                .with_info_format("application/json")
                .build(&map_clone, &wms_clone, &m.lat_lng())
                .unwrap()
                .to_string();
            log::debug!("{url:#?}");

            spawn_local({
                set_pre_text.set("Loading...".to_string());
                let abort_signal = abort_signal.get_value();
                let path = url.clone();
                async move {
                    let r = gloo::net::http::Request::get(&path)
                        .abort_signal(abort_signal.as_ref())
                        .send()
                        .await
                        .map_err(|e| log::error!("{e}"))
                        .ok()
                        .unwrap()
                        .text()
                        .await
                        .ok()
                        .unwrap();
                    log::debug!("{r}");
                    let result = r.parse::<GeoJson>().ok();
                    set_pre_text.set(format!("{result:#?}"));
                }
            });
        });
        events
    };

    let options = StoredValue::new_local(options);

    view! {
        <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
            <TileLayerWms url="https://ows.terrestris.de/osm/service" options=options>
                <TileLayerWmsEvents map_events=map_events/>
            </TileLayerWms>
        </MapContainer>
        <pre>
            { pre_text }
        </pre>
    }
}
