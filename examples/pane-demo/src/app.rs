use leptos::prelude::*;
use leptos_leaflet::prelude::*;

/// Pane Demo - Demonstrates the use of PaneContext with Popup and Tooltip components
///
/// This example shows how:
/// 1. Different panes can be created with custom z-index values for layering control
/// 2. Popup and Tooltip components automatically inherit the pane context from their parent Pane
/// 3. Custom renderers (SVG/Canvas) can be specified for vector layers within panes
/// 4. Components can override the pane context by specifying their own pane parameter
#[component]
pub fn App() -> impl IntoView {
    // State for controlling which panes are visible
    let (show_background_pane, set_show_background_pane) = signal(true);
    let (show_middle_pane, set_show_middle_pane) = signal(true);
    let (show_foreground_pane, set_show_foreground_pane) = signal(true);
    let (show_svg_renderer_pane, set_show_svg_renderer_pane) = signal(true);
    let (show_canvas_renderer_pane, set_show_canvas_renderer_pane) = signal(true);

    // Z-index values for demonstration (Leaflet defaults: tiles=200, overlays=400, shadows=500, markers=600, popups=700)
    let background_z_index = 350.0;
    let middle_z_index = 550.0;
    let foreground_z_index = 600.0;
    let svg_renderer_z_index = 700.0;
    let canvas_renderer_z_index = 750.0;

    view! {
        <div>
            // Controls section
            <div class="controls">
                <h3>"Pane Controls"</h3>
                <div class="section">
                    <h4>"Default Renderer Panes"</h4></div>
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

                <div class="section">
                    <h4>"Custom Renderer Panes"</h4>
                </div>
                <div class="control-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=show_svg_renderer_pane
                            on:change=move |ev| {
                                set_show_svg_renderer_pane.set(event_target_checked(&ev));
                            }
                        />
                        "SVG Renderer Pane (Purple Shapes)"
                    </label>
                    <span class="z-index-info">"z-index: 700"</span>
                </div>
                <div class="control-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=show_canvas_renderer_pane
                            on:change=move |ev| {
                                set_show_canvas_renderer_pane.set(event_target_checked(&ev));
                            }
                        />
                        "Canvas Renderer Pane (Cyan Shapes)"
                    </label>
                    <span class="z-index-info">"z-index: 750"</span>
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
                                // Uses PaneStrategy::Context (default) - inherits the background-pane
                                <Tooltip>"Background Circle 1 (z-index: 350) - Using Context Strategy"</Tooltip>
                            </Circle>
                            <Circle
                                center=position!(51.505, -0.09)
                                radius=300.0
                                color="darkblue"
                                fill_color="blue"
                                fill_opacity=0.3
                            >
                                // Uses explicit PaneStrategy::Custom to override the pane context
                                <Tooltip pane_strategy=PaneStrategy::Custom(Signal::derive(|| "foreground-pane".to_string()))>
                                    "Background Circle 2 - Using Custom Strategy (overridden to foreground-pane)"
                                </Tooltip>
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
                                // Uses PaneStrategy::Context (inherits middle-pane)
                                <Popup>"Red Middle Marker 1 (z-index: 550) - Context Strategy"</Popup>
                            </Marker>
                            <Marker position=position!(51.503, -0.091) icon_class="red-marker".to_string()>
                                // Uses PaneStrategy::Default to ignore pane context and use Leaflet's default
                                <Popup pane_strategy=PaneStrategy::Default>
                                    "Red Middle Marker 2 - Default Strategy (Leaflet's default popup pane)"
                                </Popup>
                            </Marker>
                            <Marker position=position!(51.507, -0.088) icon_class="red-marker".to_string()>
                                // Uses PaneStrategy::Custom with a reactive signal
                                <Popup pane_strategy=PaneStrategy::Custom(Signal::derive(|| "svg-renderer-pane".to_string()))>
                                    "Red Middle Marker 3 - Custom Strategy (svg-renderer-pane)"
                                </Popup>
                            </Marker>
                            <Circle
                                center=position!(51.505, -0.09)
                                radius=150.0
                                color="red"
                                fill_color="pink"
                                fill_opacity=0.4
                            >
                                <Tooltip>"Red Middle Circle (z-index: 550) - Context Strategy"</Tooltip>
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

                // SVG Renderer Pane - Purple shapes using explicit SVG renderer
                <Pane name="svg-renderer-pane" z_index=svg_renderer_z_index renderer=RendererType::Svg>
                {move || if show_svg_renderer_pane.get() {
                    view! {
                            <Circle
                                center=position!(51.501, -0.089)
                                radius=100.0
                                color="purple"
                                fill_color="plum"
                                fill_opacity=0.6
                            >
                                <Tooltip>"SVG Rendered Circle (z-index: 700)"</Tooltip>
                            </Circle>
                            <Polygon
                                color="indigo"
                                fill_color="violet"
                                fill_opacity=0.5
                                positions=positions(&[
                                    (51.499, -0.087),
                                    (51.503, -0.087),
                                    (51.503, -0.091),
                                    (51.499, -0.091)
                                ])
                            >
                                <Tooltip>"SVG Rendered Polygon (z-index: 700)"</Tooltip>
                            </Polygon>
                            <Polyline
                                color="purple"
                                weight=4.0
                                positions=positions(&[
                                    (51.500, -0.088),
                                    (51.502, -0.090),
                                    (51.504, -0.088),
                                    (51.506, -0.090)
                                ])
                            >
                                <Tooltip>"SVG Rendered Polyline (z-index: 700)"</Tooltip>
                            </Polyline>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
                </Pane>

                // Canvas Renderer Pane - Cyan shapes using explicit Canvas renderer
                <Pane name="canvas-renderer-pane" z_index=canvas_renderer_z_index renderer=RendererType::Svg>
                {move || if show_canvas_renderer_pane.get() {
                    view! {
                            <Circle
                                center=position!(51.509, -0.089)
                                radius=80.0
                                color="cyan"
                                fill_color="lightcyan"
                                fill_opacity=0.7
                            >
                                <Tooltip>"Canvas Rendered Circle (z-index: 750)"</Tooltip>
                            </Circle>
                            <Polygon
                                color="darkcyan"
                                fill_color="aqua"
                                fill_opacity=0.5
                                positions=positions(&[
                                    (51.507, -0.087),
                                    (51.511, -0.087),
                                    (51.511, -0.091),
                                    (51.507, -0.091)
                                ])
                            >
                                <Tooltip>"Canvas Rendered Polygon (z-index: 750)"</Tooltip>
                            </Polygon>
                            <Polyline
                                color="teal"
                                weight=3.0
                                dash_array="5, 5"
                                positions=positions(&[
                                    (51.508, -0.088),
                                    (51.510, -0.090),
                                    (51.512, -0.088),
                                    (51.514, -0.090)
                                ])
                            >
                                <Tooltip>"Canvas Rendered Polyline (z-index: 750)"</Tooltip>
                            </Polyline>
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
                    // Outside of any Pane context, PaneStrategy::Context behaves like Default
                    <Tooltip>"Default Overlay Pane Circle (z-index: 400) - Context Strategy (no context available)"</Tooltip>
                </Circle>
                <Marker position=position!(51.505, -0.09)>
                    // Demonstrates PaneStrategy::Custom outside of pane context
                    <Popup pane_strategy=PaneStrategy::Custom(Signal::derive(|| "canvas-renderer-pane".to_string()))>
                        "Default Marker - Custom Strategy (canvas-renderer-pane)"
                    </Popup>
                </Marker>

                // Standalone popup and tooltip to demonstrate PaneStrategy usage
                <Popup
                    position=position!(51.512, -0.085)
                    pane_strategy=PaneStrategy::Default
                >
                    "Standalone Popup - Default Strategy (Leaflet's default)"
                </Popup>

                <Tooltip
                    position=position!(51.498, -0.095)
                    permanent=Signal::derive(|| true)
                    pane_strategy=PaneStrategy::Custom(Signal::derive(|| "foreground-pane".to_string()))
                >
                    "Standalone Tooltip - Custom Strategy (foreground-pane)"
                </Tooltip>
            </MapContainer>
        </div>
    }
}
