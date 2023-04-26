use leptos::*;
use leptos_leaflet::{components::*, pos_opt, position, positions};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <MapContainer style="height: 400px" center=pos_opt!(51.505, -0.09) zoom=13.0 set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"/>
            <Marker position=position!(51.49, -0.08) >
                <Popup>
                    <strong>{"A pretty CSS3 popup"}</strong>
                </Popup>

            </Marker>
            <Tooltip position=position!(51.5, -0.06) permanent=true direction="top">
                <strong>{"And a tooltip"}</strong>
            </Tooltip>
            <Polyline positions=positions(&[(51.51, -0.12), (51.51, -0.13), (51.53, -0.13)])/>
        </MapContainer>
    }
}
