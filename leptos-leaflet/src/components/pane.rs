use leaflet::{CanvasOptions, SvgOptions};
use leptos::context::Provider;
use leptos::prelude::*;
use web_sys::HtmlElement;

use super::use_leaflet_context;
use crate::core::JsStoredValue;
use tracing::debug;

/// Strategy for determining which pane to use for popup and tooltip rendering.
///
/// This enum allows users to explicitly choose how popups and tooltips determine their pane:
/// - Use a custom pane name
/// - Use the pane context from parent components
/// - Use Leaflet's default behavior
#[derive(Debug, Clone, PartialEq)]
pub enum PaneStrategy {
    /// Use a custom pane name (can be reactive)
    Custom(Signal<String>),
    /// Use the pane context from parent Pane components
    Context,
    /// Use Leaflet's default pane behavior
    Default,
}

impl Default for PaneStrategy {
    fn default() -> Self {
        Self::Context
    }
}

/// Specifies the rendering scope for vector layers within a pane
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneRendererScope {
    /// Use the map's global renderer (no pane-specific renderer created)
    Global,
    /// Create a pane-specific SVG renderer for all child vector layers
    PaneSpecificSvg,
    /// Create a pane-specific Canvas renderer for all child vector layers
    PaneSpecificCanvas,
}

impl Default for PaneRendererScope {
    fn default() -> Self {
        Self::Global
    }
}

/// Context for pane information that child components can use
#[derive(Debug, Clone)]
pub struct PaneContext {
    name: String,
    renderer_scope: PaneRendererScope,
    svg_renderer: JsStoredValue<Option<leaflet::Svg>>,
    canvas_renderer: JsStoredValue<Option<leaflet::Canvas>>,
}

impl PaneContext {
    pub fn new(name: String) -> Self {
        Self {
            name,
            renderer_scope: PaneRendererScope::Global,
            svg_renderer: JsStoredValue::new_local(None),
            canvas_renderer: JsStoredValue::new_local(None),
        }
    }

    pub fn new_with_renderer(
        name: String,
        renderer_scope: PaneRendererScope,
        svg_renderer: JsStoredValue<Option<leaflet::Svg>>,
        canvas_renderer: JsStoredValue<Option<leaflet::Canvas>>,
    ) -> Self {
        Self {
            name,
            renderer_scope,
            svg_renderer,
            canvas_renderer,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn renderer_scope(&self) -> &PaneRendererScope {
        &self.renderer_scope
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
    renderer_scope: PaneRendererScope,
    svg_renderer: JsStoredValue<Option<leaflet::Svg>>,
    canvas_renderer: JsStoredValue<Option<leaflet::Canvas>>,
) -> PaneContext {
    let context =
        PaneContext::new_with_renderer(name, renderer_scope, svg_renderer, canvas_renderer);
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
///             <Pane name="svg-pane" z_index=Signal::derive(|| 600.0) renderer=PaneRendererScope::PaneSpecificSvg>
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
///             <Pane name="canvas-pane" z_index=Signal::derive(|| 650.0) renderer=PaneRendererScope::PaneSpecificCanvas>
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
    /// Renderer scope for this pane (optional)
    /// - None (default): Uses the map's global renderer (no pane-specific renderer)
    /// - PaneRendererScope::PaneSpecificSvg: Creates a pane-specific SVG renderer for child vector layers
    /// - PaneRendererScope::PaneSpecificCanvas: Creates a pane-specific Canvas renderer for child vector layers
    #[prop(into, optional)]
    renderer: Option<PaneRendererScope>,
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

    let renderer_scope = renderer.unwrap_or(PaneRendererScope::Global);

    // Provide context for children - this is the key fix!
    // Each Pane provides its own context to its children
    debug!(
        "Providing pane context for: {} with renderer scope: {:?}",
        name_clone, renderer_scope
    );
    let _pane_context = provide_pane_context_with_renderer(
        name_clone.clone(),
        renderer_scope,
        svg_renderer.clone(),
        canvas_renderer.clone(),
    );

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            debug!(
                "Creating pane: {} with renderer scope: {:?}",
                name, renderer_scope
            );

            // Create the pane
            let pane = map.create_pane_by_name(&name);

            // Set z-index if provided
            if let Some(z_idx) = z_index {
                let z_value = z_idx.get_untracked();
                let _ = pane.style().set_property("z-index", &z_value.to_string());
                debug!("Set z-index {} for pane: {}", z_value, name);
            }

            // Create and add renderer to map based on scope
            match &renderer_scope {
                PaneRendererScope::PaneSpecificSvg => {
                    let options = SvgOptions::default();
                    options.set_pane(name.clone());
                    let renderer = leaflet::Svg::with_options(&options);
                    renderer.add_to(&map);
                    svg_renderer.set_value(Some(renderer));
                    debug!(
                        "Created and stored pane-specific SVG renderer for pane: {}",
                        name
                    );
                }
                PaneRendererScope::PaneSpecificCanvas => {
                    let options = CanvasOptions::default();
                    options.set_pane(name.clone());
                    let renderer = leaflet::Canvas::with_options(&options);
                    renderer.add_to(&map);
                    canvas_renderer.set_value(Some(renderer));
                    debug!(
                        "Created and stored pane-specific Canvas renderer for pane: {}",
                        name
                    );
                }
                PaneRendererScope::Global => {
                    // Use global rendering - no pane-specific renderer needed
                    debug!("Using global renderer for pane: {}", name);
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
        assert_eq!(*context.renderer_scope(), PaneRendererScope::Global);
    }

    #[test]
    fn test_pane_context_clone() {
        let context = PaneContext::new("test-pane".to_string());
        let cloned = context.clone();
        assert_eq!(context.name(), cloned.name());
        assert_eq!(context.renderer_scope(), cloned.renderer_scope());
    }

    #[test]
    fn test_pane_context_with_svg_renderer() {
        let svg_renderer = JsStoredValue::new_local(None);
        let canvas_renderer = JsStoredValue::new_local(None);
        let context = PaneContext::new_with_renderer(
            "svg-pane".to_string(),
            PaneRendererScope::PaneSpecificSvg,
            svg_renderer,
            canvas_renderer,
        );
        assert_eq!(context.name(), "svg-pane");
        assert_eq!(
            *context.renderer_scope(),
            PaneRendererScope::PaneSpecificSvg
        );
    }

    #[test]
    fn test_pane_context_with_canvas_renderer() {
        let svg_renderer = JsStoredValue::new_local(None);
        let canvas_renderer = JsStoredValue::new_local(None);
        let context = PaneContext::new_with_renderer(
            "canvas-pane".to_string(),
            PaneRendererScope::PaneSpecificCanvas,
            svg_renderer,
            canvas_renderer,
        );
        assert_eq!(context.name(), "canvas-pane");
        assert_eq!(
            *context.renderer_scope(),
            PaneRendererScope::PaneSpecificCanvas
        );
    }

    #[test]
    fn test_renderer_scope_copy() {
        let renderer_scope = PaneRendererScope::PaneSpecificSvg;
        let copied = renderer_scope;
        assert_eq!(renderer_scope, copied);
    }

    #[test]
    fn test_renderer_scope_debug() {
        let svg = PaneRendererScope::PaneSpecificSvg;
        let canvas = PaneRendererScope::PaneSpecificCanvas;
        let global = PaneRendererScope::Global;

        assert_eq!(format!("{:?}", svg), "PaneSpecificSvg");
        assert_eq!(format!("{:?}", canvas), "PaneSpecificCanvas");
        assert_eq!(format!("{:?}", global), "Global");
    }

    #[test]
    fn test_renderer_scope_equality() {
        assert_eq!(
            PaneRendererScope::PaneSpecificSvg,
            PaneRendererScope::PaneSpecificSvg
        );
        assert_eq!(
            PaneRendererScope::PaneSpecificCanvas,
            PaneRendererScope::PaneSpecificCanvas
        );
        assert_eq!(PaneRendererScope::Global, PaneRendererScope::Global);
        assert_ne!(
            PaneRendererScope::PaneSpecificSvg,
            PaneRendererScope::PaneSpecificCanvas
        );
    }

    #[test]
    fn test_renderer_scope_default() {
        assert_eq!(PaneRendererScope::default(), PaneRendererScope::Global);
    }

    #[test]
    fn pane_strategy_default() {
        assert_eq!(PaneStrategy::default(), PaneStrategy::Context);
    }

    #[test]
    fn pane_strategy_debug() {
        let custom = PaneStrategy::Custom(Signal::derive(|| "test-pane".to_string()));
        let context = PaneStrategy::Context;
        let default = PaneStrategy::Default;

        assert_eq!(format!("{:?}", context), "Context");
        assert_eq!(format!("{:?}", default), "Default");
        // Custom variant will have a complex debug representation due to Signal
        assert!(format!("{:?}", custom).contains("Custom"));
    }

    #[test]
    fn pane_strategy_clone() {
        let context = PaneStrategy::Context;
        let cloned = context.clone();
        assert_eq!(context, cloned);

        let default = PaneStrategy::Default;
        let cloned_default = default.clone();
        assert_eq!(default, cloned_default);
    }

    #[test]
    fn pane_strategy_equality() {
        assert_eq!(PaneStrategy::Context, PaneStrategy::Context);
        assert_eq!(PaneStrategy::Default, PaneStrategy::Default);
        assert_ne!(PaneStrategy::Context, PaneStrategy::Default);
    }

    #[test]
    fn pane_strategy_custom_with_signal() {
        let signal1 = Signal::derive(|| "pane1".to_string());
        let signal2 = Signal::derive(|| "pane2".to_string());

        let custom1 = PaneStrategy::Custom(signal1);
        let custom2 = PaneStrategy::Custom(signal2);

        // Custom strategies with different signals should not be equal
        // Note: This test verifies the enum structure, actual signal comparison
        // would need runtime evaluation
        match (&custom1, &custom2) {
            (PaneStrategy::Custom(_), PaneStrategy::Custom(_)) => {
                // Both are Custom variants - this is what we expect
            }
            _ => panic!("Expected both to be Custom variants"),
        }
    }
}
