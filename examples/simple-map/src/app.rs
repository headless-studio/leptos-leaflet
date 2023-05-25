use std::time::Duration;

use leptos::*;
use leptos_leaflet::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (marker_position, set_marker_position) = create_signal(cx, Position::new(51.49, -0.08));

    let (marker_opacity, set_marker_opacity) = create_signal(cx, 1.0_f64);

    let (marker_rotation, set_marker_rotation) = create_signal(cx, 0.0);

    let (points, set_points) = create_signal(
        cx,
        vec![
            Position::new(51.505, -0.09),
            Position::new(51.51, -0.1),
            Position::new(51.51, -0.12),
        ],
    );

    create_effect(cx, move |_| {
        set_interval_with_handle(
            move || {
                set_marker_position.update(|pos| {
                    pos.lat += 0.001;
                    pos.lng += 0.001;
                });
                set_points.update(|points| {
                    points[0].lat += 0.00001;
                    points[0].lng += 0.00001;
                });
                set_marker_rotation.update(|rotation| { *rotation += 5.0; *rotation %= 360.0; });
                set_marker_opacity.update(|opacity| *opacity += 0.1);
            },
            Duration::from_millis(200),
        )
        .ok()
    });

    let opacity = Signal::derive(cx, move || (marker_opacity.get().sin() + 1.0) * 0.5);

    let mouse_events = MouseEvents::new().on_click(|e| {
        log!("Clicked on {:?}", e);
    });

    view! { cx,
          <MapContainer style="height: 400px" center=position!(51.505, -0.09) zoom=13.0 set_view=true>
              <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
              <Marker position=marker_position opacity=opacity>
                  <Popup>
                      <strong>{"A pretty CSS3 popup"}</strong>
                  </Popup>
              </Marker>
                <Marker position=position!(51.5, -0.065) draggable=true rotation=marker_rotation >
                  <Popup>
                      <strong>{"A pretty CSS3 popup"}</strong>
                  </Popup>
              </Marker>
              <Tooltip position=position!(51.5, -0.052) permanent=true direction="top">
                  <strong>{"And a tooltip"}</strong>
              </Tooltip>
              <Polyline positions=points />
              <Polygon color="purple" positions=positions(&[ (51.515, -0.09), (51.52, -0.1), (51.52, -0.12)]) >
                <Tooltip sticky=true direction="top">
                    <strong>{"I'm a polygon"}</strong>
                </Tooltip>
            </Polygon>
            <Circle center=position!(51.505, -0.09) color="blue" radius=200.0 mouse_events=mouse_events>
                <Tooltip sticky=true>{"I'm a circle"}</Tooltip>
            </Circle>
        </MapContainer>
    }
}
