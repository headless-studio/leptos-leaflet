use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::core::{IntoThreadSafeJsValue, JsSignal};

use super::{
    use_pane_context, LeafletMapContext, LeafletOverlayContainerContext, PaneStrategy, Position,
};

/// A tooltip component for displaying hover information on map elements.
///
/// This component supports different pane handling strategies through the `pane_strategy` parameter.
///
/// # Pane Integration
///
/// The `pane_strategy` parameter controls how the tooltip determines which pane to use:
/// - **PaneStrategy::Context** (default): Uses the pane context from parent `Pane` components
/// - **PaneStrategy::Custom(signal)**: Uses a specific pane name (can be reactive)
/// - **PaneStrategy::Default**: Uses Leaflet's default tooltip pane behavior
///
/// # Examples
///
/// Basic tooltip using pane context (default behavior):
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
///                 <Circle center=position!(51.505, -0.09) radius=200.0 color="blue">
///                     // This tooltip will automatically use "custom-pane" (default behavior)
///                     <Tooltip>"Hover info in custom pane"</Tooltip>
///                 </Circle>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Tooltip with explicit pane strategy:
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
///                 <Polygon positions=positions(&[(51.500, -0.086), (51.510, -0.086), (51.510, -0.096)])>
///                     // This tooltip uses a custom pane, ignoring the parent pane context
///                     <Tooltip pane_strategy=PaneStrategy::Custom(Signal::derive(|| "tooltip-pane".to_string()))>
///                         "I'm in a different pane than my parent!"
///                     </Tooltip>
///                 </Polygon>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Tooltip using Leaflet's default behavior:
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
///                 <Circle center=position!(51.505, -0.09) radius=200.0 color="blue">
///                     // This tooltip ignores both custom and context panes, using Leaflet's default
///                     <Tooltip pane_strategy=PaneStrategy::Default>
///                         "I'm in Leaflet's default tooltip pane!"
///                     </Tooltip>
///                 </Circle>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Standalone tooltip with position:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn StandaloneTooltipExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="tooltip-pane" z_index=Signal::derive(|| 650.0)>
///                 // Standalone tooltip that inherits the pane context (default behavior)
///                 <Tooltip position=position!(51.505, -0.09) permanent=Signal::derive(|| true)>
///                     "Standalone tooltip in custom pane"
///                 </Tooltip>
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// # Props
///
/// - `pane_strategy`: Controls how the tooltip determines which pane to use
/// - Other props control tooltip behavior like position, permanence, direction, etc.
#[component]
pub fn Tooltip(
    #[prop(into, optional)] position: JsSignal<Position>,
    #[prop(into, optional)] pane_strategy: Option<PaneStrategy>,
    #[prop(into, optional)] permanent: Signal<bool>,
    #[prop(into, optional, default="auto".into())] direction: Signal<String>,
    #[prop(into, optional)] sticky: Signal<bool>,
    #[prop(into, optional, default=0.9.into())] opacity: Signal<f64>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>();
    let overlay_context = use_context::<LeafletOverlayContainerContext>();

    let content = NodeRef::<Div>::new();
    // let content = view! { <div>{children()}</div>};
    Effect::new(move |_| {
        let options = leaflet::TooltipOptions::default();

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

        options.set_permanent(permanent.get_untracked());
        options.set_direction(direction.get_untracked());
        options.set_sticky(sticky.get_untracked());
        options.set_opacity(opacity.get_untracked());

        if let Some(overlay_context) = overlay_context {
            if let Some(layer) = overlay_context.container::<leaflet::Layer>() {
                let tooltip = leaflet::Tooltip::new(&options, Some(layer.unchecked_ref()))
                    .into_thread_safe_js_value();
                let content = content.get_untracked().expect("content ref");
                tooltip.set_content(content.unchecked_ref());
                layer.bind_tooltip(&tooltip);
                on_cleanup(move || {
                    tooltip.remove();
                });
            }
        } else if let Some(map) = map_context.expect("Map context not found").map() {
            let tooltip = leaflet::Tooltip::new_with_lat_lng(
                &position.get_untracked().as_lat_lng(),
                &options,
            )
            .into_thread_safe_js_value();
            let content = content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            tooltip.set_content(html_view);
            tooltip.open_on(&map);
            on_cleanup(move || {
                tooltip.remove();
            });
        }
    });

    view! { <div style="visibility:collapse"><div node_ref=content>{children()}</div></div> }
}
