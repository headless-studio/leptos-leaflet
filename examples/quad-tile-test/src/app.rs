use leptos::prelude::*;
use leptos_leaflet::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (use_quad_tiles, set_use_quad_tiles) = RwSignal::new(false).split();
    
    view! {
        <div>
            <div style="margin: 10px;">
                <label>
                    <input 
                        type="checkbox"
                        checked=use_quad_tiles
                        on:change=move |ev| {
                            set_use_quad_tiles.set(event_target_checked(&ev));
                        }
                    />
                    " Use QuadTileLayer (Bing Maps)"
                </label>
            </div>
            
            <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
                <Show
                    when=move || use_quad_tiles.get()
                    fallback=move || view! {
                        <TileLayer 
                            url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" 
                            attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
                    }
                >
                    <QuadTileLayer 
                        url="https://ecn.t0.tiles.virtualearth.net/tiles/r{q}.png?g=1" 
                        attribution="&copy; Microsoft Corporation"/>
                </Show>
                
                <Marker position=position!(51.505, -0.09) >
                    <Popup>
                        <strong>{"Toggle between standard and quad tile layers"}</strong>
                        <br/>
                        {move || if use_quad_tiles.get() { 
                            "Currently using QuadTileLayer with Bing Maps"
                        } else { 
                            "Currently using TileLayer with OpenStreetMap"
                        }}
                    </Popup>
                </Marker>
            </MapContainer>
        </div>
    }
}
