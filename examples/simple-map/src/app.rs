use leptos::*;
use leptos_leaflet::{components::*, position};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <MapContainer style="height: 400px" locate=true set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"/>
            <Marker position=position!(39.020959, -9.149168) >
                <Popup>
                    <strong>{"A pretty CSS3 popup"}</strong>
                </Popup>

            </Marker>
            <Tooltip position=position!(39.020959, -9.148168) permanent=true direction="top">
                <strong>{"And a tooltip"}</strong>
            </Tooltip>
        </MapContainer>
    }
}
