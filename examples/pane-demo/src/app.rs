use leptos::prelude::*;
use leptos_leaflet::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // State for controlling which panes are visible
    let (show_background_pane, set_show_background_pane) = signal(true);
    let (show_middle_pane, set_show_middle_pane) = signal(true);
    let (show_foreground_pane, set_show_foreground_pane) = signal(true);

    // Z-index values for demonstration (Leaflet defaults: tiles=200, overlays=400, shadows=500, markers=600, popups=700)
    let background_z_index = 350.0;
    let middle_z_index = 550.0;
    let foreground_z_index = 600.0;

    view! {
        <div>
            // Controls section
            <div class="controls">
                <h3>"Pane Controls"</h3>
                <div class="control-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=show_background_pane
                            on:change=move |ev| {
                                set_show_background_pane.set(event_target_checked(&ev));
                            }
                        />
                        "Background Pane (Blue Circles)"
                    </label>
                    <span class="z-index-info">"z-index: 350"</span>
                </div>
                <div class="control-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=show_middle_pane
                            on:change=move |ev| {
                                set_show_middle_pane.set(event_target_checked(&ev));
                            }
                        />
                        "Middle Pane (Red Markers)"
                    </label>
                    <span class="z-index-info">"z-index: 550"</span>
                </div>
                <div class="control-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=show_foreground_pane
                            on:change=move |ev| {
                                set_show_foreground_pane.set(event_target_checked(&ev));
                            }
                        />
                        "Foreground Pane (Green Polygons)"
                    </label>
                    <span class="z-index-info">"z-index: 650"</span>
                </div>
            </div>

            // Map container
            <MapContainer
                class="map-container"
                center=Position::new(51.505, -0.09)
                zoom=13.0
                set_view=true
            >
                // Base tile layer
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />

                // Background Pane - Blue Circles (lowest z-index)
                <Pane name="background-pane" z_index=background_z_index>
                {move || if show_background_pane.get() {
                    view! {
                            <Circle
                                center=position!(51.505, -0.09)
                                radius=400.0
                                color="blue"
                                fill_color="lightblue"
                                fill_opacity=0.5
                            >
                                <Tooltip>"Background Circle 1 (z-index: 350)"</Tooltip>
                            </Circle>
                            <Circle
                                center=position!(51.505, -0.09)
                                radius=300.0
                                color="darkblue"
                                fill_color="blue"
                                fill_opacity=0.3
                            >
                                <Tooltip>"Background Circle 2 (z-index: 350)"</Tooltip>
                            </Circle>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
                </Pane>

                // Middle Pane - Red Markers (middle z-index)
                <Pane name="middle-pane" z_index=middle_z_index>
                {move || if show_middle_pane.get() {
                    view! {
                            <Marker position=position!(51.505, -0.09) icon_class="red-marker".to_string()>
                                <Popup>"Red Middle Marker 1 (z-index: 550)"</Popup>
                            </Marker>
                            <Marker position=position!(51.503, -0.091) icon_class="red-marker".to_string()>
                                <Popup>"Red Middle Marker 2 (z-index: 550)"</Popup>
                            </Marker>
                            <Marker position=position!(51.507, -0.088) icon_class="red-marker".to_string()>
                                <Popup>"Red Middle Marker 3 (z-index: 550)"</Popup>
                            </Marker>
                            <Circle
                                center=position!(51.505, -0.09)
                                radius=150.0
                                color="red"
                                fill_color="pink"
                                fill_opacity=0.4
                            >
                                <Tooltip>"Red Middle Circle (z-index: 550)"</Tooltip>
                            </Circle>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
                </Pane>

                // Foreground Pane - Green Polygons (highest z-index)
                <Pane name="foreground-pane" z_index=foreground_z_index>
                {move || if show_foreground_pane.get() {
                    view! {
                            <Polygon
                                color="green"
                                fill_color="lightgreen"
                                fill_opacity=0.7
                                positions=positions(&[
                                    (51.500, -0.086),
                                    (51.510, -0.086),
                                    (51.510, -0.096),
                                    (51.500, -0.096)
                                ])
                            >
                                <Tooltip>"Foreground Polygon 1 (z-index: 650)"</Tooltip>
                            </Polygon>
                            <Polygon
                                color="darkgreen"
                                fill_color="green"
                                fill_opacity=0.6
                                positions=positions(&[
                                    (51.502, -0.088),
                                    (51.508, -0.088),
                                    (51.508, -0.094),
                                    (51.502, -0.094)
                                ])
                            >
                                <Tooltip>"Foreground Polygon 2 (z-index: 650)"</Tooltip>
                            </Polygon>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
                </Pane>

                // Add additional elements outside of panes for comparison
                <Circle
                    center=position!(51.505, -0.09)
                    radius=200.0
                    color="orange"
                    fill_color="yellow"
                    fill_opacity=0.4
                >
                    <Tooltip>"Default Overlay Pane Circle (z-index: 400)"</Tooltip>
                </Circle>
                <Marker position=position!(51.505, -0.09)>
                    <Popup>"Default Marker Pane (z-index: 600)"</Popup>
                </Marker>
            </MapContainer>
        </div>
    }
}
