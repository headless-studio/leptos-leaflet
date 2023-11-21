use geojson::GeoJson;
use leaflet::{TileLayerWmsOptions, WmsRequestBuilder};
use leptos::*;
use leptos_leaflet::*;

#[component]
pub fn App() -> impl IntoView {
    let (pre_text, set_pre_text) = create_signal("Please click into the map to start a request.".to_string());

    let options = TileLayerWmsOptions::new();
    options.set_layers("OSM-WMS".to_string());

    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let action_details = create_action(move |path: &String| {
        set_pre_text.set("Loading...".to_string());
        let abort_signal = abort_signal.clone();
        let path = path.clone();
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
            r.parse::<GeoJson>().ok()
        }
    });
    let action_details_value = action_details.value();
    let update_action_details_value = move || {
        if let Some(val) = action_details_value.get() {
            set_pre_text.set(format!("{val:#?}"));
        }
    };
    let map_events = move |map: &leaflet::Map, wms: &leaflet::TileLayerWms| {
        let mut events = MapEvents::new();
        let map_clone = map.clone();
        let wms_clone  = wms.clone();
        events = events.mouse_click(move |m| {
            let url = WmsRequestBuilder::default()
                .with_info_format("application/json")
                .build(&map_clone, &wms_clone, &m.lat_lng())
                .unwrap()
                .to_string();
            log::debug!("{url:#?}");
            action_details.dispatch(url);
        });
        events
    };


    view! {
        <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
            <TileLayerWms url="https://ows.terrestris.de/osm/service" options=options>
                <TileLayerWmsEvents map_events=map_events/>
            </TileLayerWms>
        </MapContainer>
        <pre>
            { pre_text }
        </pre>
        { update_action_details_value }
    }
}
