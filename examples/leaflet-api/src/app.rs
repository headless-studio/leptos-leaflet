use std::time::Duration;

use leptos::prelude::*;
use leptos_leaflet::leaflet::{self, DragEndEvent, Event, Evented, LatLng, MarkerOptions};
use leptos_leaflet::prelude::*;
use web_sys::wasm_bindgen::JsCast;

use crate::droppables::{DroppableMarker, DRAG_END_EVENT, DRAG_EVENT};

#[component]
pub fn App() -> impl IntoView {
    let map = RwSignal::new_local(None);
    let markers = RwSignal::new_local(Vec::<DroppableMarker>::new());

    let coordinate_one = RwSignal::new_local((51.5, -0.08));
    let coordinate_two = RwSignal::new_local((51.5, -0.06));

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

    Effect::new(move |_ignore: Option<()>| {
        map.track();
        if let Some(map) = map.get() {
            let options = MarkerOptions::new();
            options.set_draggable(true);
            let mut marker_one =
                DroppableMarker::new(coordinate_one.get_untracked(), Some(options.clone()));
            let mut marker_two =
                DroppableMarker::new(coordinate_two.get_untracked(), Some(options));
            marker_one.add_to(&map);
            marker_two.add_to(&map);
            let drag_end = Evented::on_leaflet_event(
                marker_one.marker_ref(),
                DRAG_END_EVENT,
                move |e: DragEndEvent| {
                    let m = e.target().unchecked_into::<leaflet::Marker>();
                    let lat_lng = m.get_lat_lng();
                    coordinate_one.set((lat_lng.lat(), lat_lng.lng()));
                },
            );
            marker_one.add_event(drag_end);

            let drag =
                Evented::on_leaflet_event(marker_two.marker_ref(), DRAG_EVENT, move |e: Event| {
                    let m = e.target().unchecked_into::<leaflet::Marker>();
                    let lat_lng = m.get_lat_lng();
                    coordinate_two.set((lat_lng.lat(), lat_lng.lng()));
                });
            marker_two.add_event(drag);

            let base_marker = marker_one.marker();

            // Comment this and markers will be removed from the map right away
            markers.update(|markers| {
                markers.push(marker_one);
                markers.push(marker_two);
            });

            set_timeout(
                move || {
                    let lat_lng = LatLng::new(51.5, -0.1);
                    base_marker.set_lat_lng(&lat_lng);
                    coordinate_one.set((lat_lng.lat(), lat_lng.lng()));
                },
                Duration::from_secs(2),
            );
        }
    });

    view! {
      <MapContainer map=map.write_only() style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
          <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
      </MapContainer>
      <div><span>"Coordinate DragEnd: "{move || {let c = coordinate_one.get(); format!("{:.4}, {:.4}", c.0, c.1)}}" "</span></div>
      <div><span>"Coordinate Drag: "{move || {let c = coordinate_two.get(); format!("{:.4}, {:.4}", c.0, c.1)}}" "</span></div>
      <div><button on:click=add_marker>{"Add marker!"}</button>" "<button on:click=remove_marker>{"Remove last added marker!"} </button></div>
    }
}
