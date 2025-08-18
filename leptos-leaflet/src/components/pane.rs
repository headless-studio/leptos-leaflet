use leptos::html::Div;
use leptos::prelude::*;
use web_sys::HtmlElement;

use super::use_leaflet_context;
use crate::core::JsStoredValue;

/// Context for pane information that child components can use
#[derive(Debug, Clone)]
pub struct PaneContext {
    name: String,
}

impl PaneContext {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Provides pane context for child components
pub fn provide_pane_context(name: String) -> PaneContext {
    let context = PaneContext::new(name);
    provide_context(context.clone());
    context
}

/// Gets the current pane context if available
pub fn use_pane_context() -> Option<PaneContext> {
    use_context::<PaneContext>()
}

/// A Leaflet pane component that creates a custom map pane for organizing layers with custom z-index ordering.
///
/// Panes allow you to control the stacking order of different map layers. Child components of this pane
/// will automatically use this pane unless they specify their own pane name.
///
/// # Examples
///
/// Basic usage with custom z-index:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let z_index = Signal::derive(|| 550.0);
///
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="custom-pane" z_index=z_index>
///                 <Circle
///                     center=position!(51.505, -0.09)
///                     radius=300.0
///                     color="red"
///                     fill_color="pink"
///                 />
///                 <Marker position=position!(51.506, -0.088) />
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// Multiple panes with different z-indices:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn MultiPaneExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             // Background pane (lowest z-index)
///             <Pane name="background" z_index=Signal::derive(|| 350.0)>
///                 <Circle
///                     center=position!(51.505, -0.09)
///                     radius=500.0
///                     color="blue"
///                     fill_opacity=0.3
///                 />
///             </Pane>
///
///             // Foreground pane (higher z-index)
///             <Pane name="foreground" z_index=Signal::derive(|| 650.0)>
///                 <Polygon
///                     positions=positions(&[(51.500, -0.086), (51.510, -0.086), (51.510, -0.096)])
///                     color="green"
///                     fill_color="lightgreen"
///                 />
///             </Pane>
///         </MapContainer>
///     }
/// }
/// ```
///
/// # Default Z-Index Values
///
/// Leaflet uses the following default z-index values:
/// - Tile layers: 200
/// - Overlay pane (default for vector layers): 400
/// - Shadow pane: 500
/// - Marker pane: 600
/// - Popup pane: 700
///
/// Choose your custom z-index values relative to these defaults.
#[component(transparent)]
pub fn Pane(
    /// Name of the pane (must be unique)
    #[prop(into)]
    name: String,
    /// Z-index for the pane (optional, controls stacking order)
    #[prop(into, optional)]
    z_index: Option<Signal<f64>>,
    /// Child components that should be rendered in this pane
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let map_context = use_leaflet_context().expect("Pane must be used within a MapContainer");
    let name_clone = name.clone();

    // Store the pane element
    let pane_element = JsStoredValue::new_local(None::<HtmlElement>);

    // Create a node ref to hold the children content
    let content_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            // Get the map container to use as the parent for the pane
            let container = map.get_pane_by_name("mapPane");

            // Create the pane
            let pane = map.create_pane(&name, &container);

            // Set z-index if provided
            if let Some(z_idx) = z_index {
                let z_value = z_idx.get_untracked();
                let _ = pane.style().set_property("z-index", &z_value.to_string());
            }

            pane_element.set_value(Some(pane));
        }
    });

    // Effect to move content to pane when both are available
    let content_ref_clone = content_ref.clone();
    let pane_element_clone = pane_element.clone();
    Effect::new(move |_| {
        // Wait for both the pane and content to be available
        if let (Some(pane), Some(content_div)) = (
            pane_element_clone.get_value().as_ref(),
            content_ref_clone.get(),
        ) {
            // Move all children from the hidden div to the pane
            while let Some(child) = content_div.first_child() {
                let _ = pane.append_child(&child);
            }
        }
    });

    // Watch for z-index changes
    if let Some(z_idx) = z_index {
        let z_index_effect = Effect::watch(
            move || z_idx.get(),
            move |&z_value, _, _| {
                if let Some(pane) = pane_element.get_value().as_ref() {
                    let _ = pane.style().set_property("z-index", &z_value.to_string());
                }
            },
            false,
        );

        on_cleanup(move || {
            z_index_effect.stop();
        });
    }

    on_cleanup(move || {
        if let Some(pane) = pane_element.get_value() {
            // Remove the pane from DOM
            if let Some(parent) = pane.parent_element() {
                let _ = parent.remove_child(&pane);
            }
        }
    });

    // Provide context for child components so they know which pane to use
    provide_pane_context(name_clone);

    // Render children to a hidden div, then move them to the pane
    view! {
        <div style="display: none;">
            <div node_ref=content_ref>
                {children.map(|child| child())}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pane_context_creation() {
        let context = PaneContext::new("test-pane".to_string());
        assert_eq!(context.name(), "test-pane");
    }

    #[test]
    fn test_pane_context_clone() {
        let context = PaneContext::new("test-pane".to_string());
        let cloned = context.clone();
        assert_eq!(context.name(), cloned.name());
    }
}
