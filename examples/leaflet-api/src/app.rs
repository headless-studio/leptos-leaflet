use leaflet::MarkerOptions;
use leptos::*;
use leptos_leaflet::*;

use crate::droppables::DroppableMarker;

#[component]
pub fn App() -> impl IntoView {
    let map = RwSignal::new(None);
    let markers = RwSignal::new(Vec::<DroppableMarker>::new());

    let add_marker = move |_| {
        if let Some(map) = map.get().as_ref() {
            let options = MarkerOptions::new();
            options.set_draggable(true);
            let marker = DroppableMarker::new((51.5, -0.09), Some(options));
            marker.add_to(map);
            markers.update(|markers| markers.push(marker));
        }
    };

    let remove_marker = move |_| {
        markers.update(|markers| {
            if let Some(_marker) = markers.pop() {
                // Since we are using DroppableMarker, we don't need to remove the marker manually
                // when drop is called on the end of this scope, it will remove the marker
            }
        });
    };

    view! {
      <MapContainer map=map.write_only() style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
          <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
      </MapContainer>
      <button on:click=add_marker>{"Add marker!"} </button>
      <button on:click=remove_marker>{"Remove last added marker!"} </button>
    }
}
