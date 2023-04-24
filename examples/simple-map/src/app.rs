use leptos::*;
use leptos_leaflet::{components::*, latlng};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <MapContainer style="height: 400px" locate=true set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"/>
            <Marker position=latlng!(39.020959, -9.149168) >
                <Popup>
                    <strong>{"A pretty CSS3 popup"}</strong>
                </Popup>
            </Marker>
        </MapContainer>
    }
}
