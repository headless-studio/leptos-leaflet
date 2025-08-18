use leaflet::{CanvasOptions, SvgOptions};
use leptos::context::Provider;
use leptos::prelude::*;
use web_sys::HtmlElement;

use super::use_leaflet_context;
use crate::core::JsStoredValue;
use tracing::debug;

/// Type of renderer to use for vector layers in a pane
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererType {
    /// Use Leaflet's default rendering (typically SVG)
    Default,
    /// Force SVG rendering for all child vector layers
    Svg,
    /// Force Canvas rendering for all child vector layers
    Canvas,
}

impl Default for RendererType {
    fn default() -> Self {
        Self::Default
    }
}

/// Context for pane information that child components can use
#[derive(Debug, Clone)]
pub struct PaneContext {
    name: String,
    renderer_type: RendererType,
    svg_renderer: JsStoredValue<Option<leaflet::Svg>>,
    canvas_renderer: JsStoredValue<Option<leaflet::Canvas>>,
}

impl PaneContext {
    pub fn new(name: String) -> Self {
        Self {
            name,
            renderer_type: RendererType::Default,
            svg_renderer: JsStoredValue::new_local(None),
            canvas_renderer: JsStoredValue::new_local(None),
        }
    }

    pub fn new_with_renderer(
        name: String,
        renderer_type: RendererType,
        svg_renderer: JsStoredValue<Option<leaflet::Svg>>,
        canvas_renderer: JsStoredValue<Option<leaflet::Canvas>>,
    ) -> Self {
        Self {
            name,
            renderer_type,
            svg_renderer,
            canvas_renderer,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn renderer_type(&self) -> &RendererType {
        &self.renderer_type
    }

    /// Gets the SVG renderer if available
    pub fn svg_renderer(&self) -> Option<leaflet::Svg> {
        self.svg_renderer.get_value()
    }

    /// Gets the Canvas renderer if available
    pub fn canvas_renderer(&self) -> Option<leaflet::Canvas> {
        self.canvas_renderer.get_value()
    }
}

/// Provides pane context for child components
pub fn provide_pane_context(name: String) -> PaneContext {
    let context = PaneContext::new(name);
    provide_context(context.clone());
    context
}

/// Provides pane context with renderer for child components
pub fn provide_pane_context_with_renderer(
    name: String,
    renderer_type: RendererType,
    svg_renderer: JsStoredValue<Option<leaflet::Svg>>,
    canvas_renderer: JsStoredValue<Option<leaflet::Canvas>>,
) -> PaneContext {
    let context =
        PaneContext::new_with_renderer(name, renderer_type, svg_renderer, canvas_renderer);
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
/// Usage with explicit SVG renderer:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn SvgRendererExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="svg-pane" z_index=Signal::derive(|| 600.0) renderer=RendererType::Svg>
///                 <Circle
///                     center=position!(51.505, -0.09)
///                     radius=200.0
///                     color="blue"
///                     fill_color="lightblue"
///                 />
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
/// Usage with explicit Canvas renderer:
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
///
/// #[component]
/// fn CanvasRendererExample() -> impl IntoView {
///     view! {
///         <MapContainer center=Position::new(51.505, -0.09) zoom=13.0>
///             <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" />
///
///             <Pane name="canvas-pane" z_index=Signal::derive(|| 650.0) renderer=RendererType::Canvas>
///                 <Polyline
///                     positions=positions(&[(51.505, -0.09), (51.51, -0.1), (51.51, -0.12)])
///                     color="red"
///                     weight=4.0
///                 />
///                 <Circle
///                     center=position!(51.505, -0.09)
///                     radius=150.0
///                     color="orange"
///                     fill_color="yellow"
///                 />
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
    /// Renderer type for this pane (optional)
    /// - None (default): Uses Leaflet's default rendering (typically SVG)
    /// - RendererType::Svg: Forces SVG rendering for all child vector layers
    /// - RendererType::Canvas: Forces Canvas rendering for all child vector layers
    #[prop(into, optional)]
    renderer: Option<RendererType>,
    /// Child components that should be rendered in this pane
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let map_context = use_leaflet_context().expect("Pane must be used within a MapContainer");
    let name_clone = name.clone();

    // Store the pane element
    let pane_element = JsStoredValue::new_local(None::<HtmlElement>);

    // Create renderer storage
    let svg_renderer = JsStoredValue::new_local(None::<leaflet::Svg>);
    let canvas_renderer = JsStoredValue::new_local(None::<leaflet::Canvas>);

    let renderer_type = renderer.unwrap_or(RendererType::Default);

    // Provide context for children - this is the key fix!
    // Each Pane provides its own context to its children
    debug!(
        "Providing pane context for: {} with renderer type: {:?}",
        name_clone, renderer_type
    );
    let _pane_context = provide_pane_context_with_renderer(
        name_clone.clone(),
        renderer_type,
        svg_renderer.clone(),
        canvas_renderer.clone(),
    );

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            debug!(
                "Creating pane: {} with renderer type: {:?}",
                name, renderer_type
            );

            // Create the pane
            let pane = map.create_pane_by_name(&name);

            // Set z-index if provided
            if let Some(z_idx) = z_index {
                let z_value = z_idx.get_untracked();
                let _ = pane.style().set_property("z-index", &z_value.to_string());
                debug!("Set z-index {} for pane: {}", z_value, name);
            }

            // Create and add renderer to map based on type
            match &renderer_type {
                RendererType::Svg => {
                    let options = SvgOptions::default();
                    options.set_pane(name.clone());
                    let renderer = leaflet::Svg::with_options(&options);
                    renderer.add_to(&map);
                    svg_renderer.set_value(Some(renderer));
                    debug!("Created and stored SVG renderer for pane: {}", name);
                }
                RendererType::Canvas => {
                    let options = CanvasOptions::default();
                    options.set_pane(name.clone());
                    let renderer = leaflet::Canvas::with_options(&options);
                    renderer.add_to(&map);
                    canvas_renderer.set_value(Some(renderer));
                    debug!("Created and stored Canvas renderer for pane: {}", name);
                }
                RendererType::Default => {
                    // Use default rendering - no custom renderer needed
                    debug!("Using default renderer for pane: {}", name);
                }
            }

            pane_element.set_value(Some(pane));
            debug!("Finished creating pane: {}", name);
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
        if let Some(map) = map_context.map() {
            // Remove renderers from map
            if let Some(svg) = svg_renderer.get_value() {
                svg.remove_from(&map);
            }
            if let Some(canvas) = canvas_renderer.get_value() {
                canvas.remove_from(&map);
            }
        }

        if let Some(pane) = pane_element.get_value() {
            // Remove the pane from DOM
            if let Some(parent) = pane.parent_element() {
                let _ = parent.remove_child(&pane);
            }
        }
    });

    // Render children within a Provider to ensure proper context scoping
    view! {
        <Provider value=_pane_context>
            {children.map(|child| child())}
        </Provider>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pane_context_creation() {
        let context = PaneContext::new("test-pane".to_string());
        assert_eq!(context.name(), "test-pane");
        assert_eq!(*context.renderer_type(), RendererType::Default);
    }

    #[test]
    fn test_pane_context_clone() {
        let context = PaneContext::new("test-pane".to_string());
        let cloned = context.clone();
        assert_eq!(context.name(), cloned.name());
        assert_eq!(context.renderer_type(), cloned.renderer_type());
    }

    #[test]
    fn test_pane_context_with_svg_renderer() {
        let svg_renderer = JsStoredValue::new_local(None);
        let canvas_renderer = JsStoredValue::new_local(None);
        let context = PaneContext::new_with_renderer(
            "svg-pane".to_string(),
            RendererType::Svg,
            svg_renderer,
            canvas_renderer,
        );
        assert_eq!(context.name(), "svg-pane");
        assert_eq!(*context.renderer_type(), RendererType::Svg);
    }

    #[test]
    fn test_pane_context_with_canvas_renderer() {
        let svg_renderer = JsStoredValue::new_local(None);
        let canvas_renderer = JsStoredValue::new_local(None);
        let context = PaneContext::new_with_renderer(
            "canvas-pane".to_string(),
            RendererType::Canvas,
            svg_renderer,
            canvas_renderer,
        );
        assert_eq!(context.name(), "canvas-pane");
        assert_eq!(*context.renderer_type(), RendererType::Canvas);
    }

    #[test]
    fn test_renderer_type_copy() {
        let renderer_type = RendererType::Svg;
        let copied = renderer_type;
        assert_eq!(renderer_type, copied);
    }

    #[test]
    fn test_renderer_type_debug() {
        let svg = RendererType::Svg;
        let canvas = RendererType::Canvas;
        let default = RendererType::Default;

        assert_eq!(format!("{:?}", svg), "Svg");
        assert_eq!(format!("{:?}", canvas), "Canvas");
        assert_eq!(format!("{:?}", default), "Default");
    }

    #[test]
    fn test_renderer_type_equality() {
        assert_eq!(RendererType::Svg, RendererType::Svg);
        assert_eq!(RendererType::Canvas, RendererType::Canvas);
        assert_eq!(RendererType::Default, RendererType::Default);
        assert_ne!(RendererType::Svg, RendererType::Canvas);
    }

    #[test]
    fn test_renderer_type_default() {
        assert_eq!(RendererType::default(), RendererType::Default);
    }
}
