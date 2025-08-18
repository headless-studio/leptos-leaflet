use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use super::{use_pane_context, LeafletMapContext, PaneStrategy, Position};
use crate::core::{IntoThreadSafeJsValue, JsSignal};
use crate::prelude::LeafletOverlayContainerContext;

/// A popup component for displaying content on the map.
///
/// This component supports different pane handling strategies through the `pane_strategy` parameter.
///
/// # Pane Integration
///
/// The `pane_strategy` parameter controls how the popup determines which pane to use:
/// - **PaneStrategy::Context** (default): Uses the pane context from parent `Pane` components
/// - **PaneStrategy::Custom(signal)**: Uses a specific pane name (can be reactive)
/// - **PaneStrategy::Default**: Uses Leaflet's default popup pane behavior
///
/// # Examples
///
/// Basic popup using pane context (default behavior):
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="custom-pane" z_index=Signal::derive(|| 650.0)>
///                 <Marker position=position!(51.505, -0.09)>
///                     // This popup will automatically use "custom-pane" (default behavior)
///                     <Popup>"I'm in the custom pane!"</Popup>
///                 </Marker>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Popup with explicit pane strategy:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn ExplicitPaneExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="background-pane" z_index=Signal::derive(|| 350.0)>
///                 <Marker position=position!(51.505, -0.09)>
///                     // This popup uses a custom pane, ignoring the parent pane context
///                     <Popup pane_strategy=PaneStrategy::Custom(Signal::derive(|| "popup-pane".to_string()))>
///                         "I'm in a different pane than my parent!"
///                     </Popup>
///                 </Marker>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Popup using Leaflet's default behavior:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn DefaultPaneExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="custom-pane" z_index=Signal::derive(|| 650.0)>
///                 <Marker position=position!(51.505, -0.09)>
///                     // This popup ignores both custom and context panes, using Leaflet's default
///                     <Popup pane_strategy=PaneStrategy::Default>
///                         "I'm in Leaflet's default popup pane!"
///                     </Popup>
///                 </Marker>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Standalone popup with position:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn StandalonePopupExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="popup-pane" z_index=Signal::derive(|| 700.0)>
///                 // Standalone popup that inherits the pane context (default behavior)
///                 <Popup position=position!(51.505, -0.09)>
///                     "Standalone popup in custom pane"
///                 </Popup>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
#[component]
pub fn Popup(
    #[prop(into, optional)] position: JsSignal<Position>,
    #[prop(into, optional)] pane_strategy: Option<PaneStrategy>,
    #[prop(into, optional)] offset: Option<Signal<(i32, i32)>>,
    #[prop(into, optional)] min_width: Option<Signal<f64>>,
    #[prop(into, optional)] max_width: Option<Signal<f64>>,
    #[prop(into, optional)] auto_pan: Option<Signal<bool>>,
    #[prop(into, optional)] auto_pan_padding_top_left: Option<Signal<(i32, i32)>>,
    #[prop(into, optional)] auto_pan_padding_bottom_right: Option<Signal<(i32, i32)>>,
    #[prop(into, optional)] auto_pan_padding: Option<Signal<(i32, i32)>>,
    #[prop(into, optional)] keep_in_view: Option<Signal<bool>>,
    #[prop(into, optional)] close_button: Option<Signal<bool>>,
    #[prop(into, optional)] auto_close: Option<Signal<bool>>,
    #[prop(into, optional)] close_on_escape_key: Option<Signal<bool>>,
    #[prop(into, optional)] close_on_click: Option<Signal<bool>>,
    #[prop(into, optional)] class_name: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>();
    let overlay_context = use_context::<LeafletOverlayContainerContext>();

    // Render popup content to a html element
    let content = NodeRef::<Div>::new();

    Effect::new(move |_| {
        let inner_content = content;
        let options = leaflet::PopupOptions::default();

        // Apply pane strategy
        match pane_strategy.as_ref().unwrap_or(&PaneStrategy::Context) {
            PaneStrategy::Custom(pane_signal) => {
                let pane_value = pane_signal.get_untracked();
                if !pane_value.is_empty() {
                    options.set_pane(pane_value);
                }
            }
            PaneStrategy::Context => {
                if let Some(pane_context) = use_pane_context() {
                    options.set_pane(pane_context.name().to_string());
                }
            }
            PaneStrategy::Default => {
                // Use Leaflet's default pane behavior - don't set any pane
            }
        }
        if let Some(offset) = offset {
            let (x, y) = offset.get_untracked();
            options.set_offset(leaflet::Point::new(f64::from(x), f64::from(y)));
        }
        if let Some(min_width) = min_width {
            options.set_min_width(min_width.get_untracked());
        }
        if let Some(max_width) = max_width {
            options.set_max_width(max_width.get_untracked());
        }
        if let Some(auto_pan) = auto_pan {
            options.set_auto_pan(auto_pan.get_untracked());
        }
        if let Some(auto_pan_padding_top_left) = auto_pan_padding_top_left {
            let (x, y) = auto_pan_padding_top_left.get_untracked();
            options.set_auto_pan_padding_top_left(leaflet::Point::new(f64::from(x), f64::from(y)));
        }
        if let Some(auto_pan_padding_bottom_right) = auto_pan_padding_bottom_right {
            let (x, y) = auto_pan_padding_bottom_right.get_untracked();
            options
                .set_auto_pan_padding_bottom_right(leaflet::Point::new(f64::from(x), f64::from(y)));
        }
        if let Some(auto_pan_padding) = auto_pan_padding {
            let (x, y) = auto_pan_padding.get_untracked();
            options.set_auto_pan_padding(leaflet::Point::new(f64::from(x), f64::from(y)));
        }
        if let Some(keep_in_view) = keep_in_view {
            options.set_keep_in_view(keep_in_view.get_untracked());
        }
        if let Some(close_button) = close_button {
            options.set_close_button(close_button.get_untracked());
        }
        if let Some(auto_close) = auto_close {
            options.set_auto_close(auto_close.get_untracked());
        }
        if let Some(close_on_escape_key) = close_on_escape_key {
            options.set_close_on_escape_key(close_on_escape_key.get_untracked());
        }
        if let Some(close_on_click) = close_on_click {
            options.set_close_on_click(close_on_click.get_untracked());
        }
        if let Some(class_name) = &class_name {
            options.set_class_name(class_name.get_untracked());
        }
        if let Some(overlay_context) = overlay_context {
            if let Some(marker) = overlay_context.container::<leaflet::Layer>() {
                let popup = leaflet::Popup::new(&options, Some(marker.unchecked_ref()))
                    .into_thread_safe_js_value();
                let content = inner_content.get_untracked().expect("content ref");
                let html_view: &JsValue = content.unchecked_ref();
                popup.set_content(html_view);
                marker.bind_popup(&popup);
                on_cleanup(move || {
                    popup.remove();
                });
            }
        } else if let Some(map) = map_context.expect("map context not found").map() {
            let popup =
                leaflet::Popup::new_with_lat_lng(&position.get_untracked().as_lat_lng(), &options)
                    .into_thread_safe_js_value();
            let content = inner_content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            popup.set_content(html_view);
            popup.open_on(&map);
            on_cleanup(move || {
                popup.remove();
            });
        }
    });

    view! { <div style="visibility:collapse"><div node_ref=content>{children()}</div></div> }
}
