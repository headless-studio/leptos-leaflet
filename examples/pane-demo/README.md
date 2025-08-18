# Pane Demo Example

This example demonstrates how to use the `Pane` component in leptos-leaflet to create custom map panes with different z-index values for controlling layer stacking order. It also showcases the new **PaneStrategy enum** for Popup and Tooltip components, which provides explicit control over pane handling behavior.

## What This Example Shows

- **Five Custom Panes**: Background, Middle, Foreground, SVG Renderer, and Canvas Renderer panes with different z-index values
- **Interactive Controls**: Checkboxes to toggle each pane on/off
- **Z-Index Demonstration**: Visual demonstration of how z-index affects layer stacking relative to Leaflet's default panes
- **PaneStrategy Integration**: Popup and Tooltip components demonstrate all three pane strategies:
  - `PaneStrategy::Context` (default): Inherit parent pane context
  - `PaneStrategy::Custom(signal)`: Use a specific custom pane
  - `PaneStrategy::Default`: Use Leaflet's default pane behavior
- **Custom Renderers**: SVG and Canvas renderer panes demonstrate explicit renderer types
- **Mixed Content**: Different map elements (circles, markers, polygons) in different panes with overlapping geometry

## Pane Configuration

### Background Pane (z-index: 350)
- Contains blue circles with **tooltips demonstrating different PaneStrategy options**
- Below Leaflet's default overlay pane (400), appears behind most elements
- Circle 1: Uses `PaneStrategy::Context` (default behavior)
- Circle 2: Uses `PaneStrategy::Custom` to override to a different pane

### Middle Pane (z-index: 550)  
- Contains red markers with **popups demonstrating all three PaneStrategy options**
- Between Leaflet's shadow pane (500) and marker pane (600)
- Marker 1: Uses `PaneStrategy::Context` (inherits middle-pane)
- Marker 2: Uses `PaneStrategy::Default` (ignores context, uses Leaflet's default)
- Marker 3: Uses `PaneStrategy::Custom` (explicit pane override)

### Foreground Pane (z-index: 650)
- Contains green polygons with **tooltips using the default Context strategy**
- Above Leaflet's default marker pane (600), appears on top of most elements
- Tooltips automatically use the "foreground-pane" via context inheritance

### SVG Renderer Pane (z-index: 700)
- Uses explicit SVG rendering for all vector layers
- Contains purple shapes with **tooltips using Context strategy**
- Demonstrates custom renderer integration with PaneStrategy

### Canvas Renderer Pane (z-index: 750)
- Uses explicit Canvas rendering for all vector layers
- Contains cyan shapes with **tooltips using Context strategy**
- Demonstrates custom renderer integration with PaneStrategy

### Default Elements
- An orange circle using Leaflet's default overlay pane (z-index: 400) with tooltip
- A default marker using Leaflet's default marker pane (z-index: 600) with popup
- All elements are positioned at the same center point to clearly demonstrate z-index layering

## Key Features Demonstrated

1. **Custom Z-Index**: Each pane has a different z-index to control stacking order relative to Leaflet's defaults
2. **Dynamic Visibility**: Panes can be toggled on/off using reactive signals
3. **PaneStrategy Enum**: Three explicit strategies for popup and tooltip pane handling:
   - **`PaneStrategy::Context`** (default): Inherit parent pane context automatically
   - **`PaneStrategy::Custom(signal)`**: Use a specific pane name (can be reactive)
   - **`PaneStrategy::Default`**: Ignore all context and use Leaflet's default behavior
4. **Custom Renderers**: SVG and Canvas renderer types for vector layers within panes
5. **Mixed Layer Types**: Different Leaflet layer types work within panes
6. **Geometric Overlap**: All elements overlap at the map center to clearly show layering effects
7. **Standalone Components**: Popup and Tooltip examples outside of pane context with custom strategies

## Running the Example

```bash
# Navigate to the example directory
cd examples/pane-demo

# Install dependencies and build
cargo check

# Serve the example (you'll need a local server)
# For example, using Python:
python -m http.server 8000

# Or using Node.js:
npx serve .

# Then open http://localhost:8000 in your browser
```

## Code Structure

- `src/main.rs`: Entry point that mounts the app
- `src/app.rs`: Main application component with pane logic
- `index.html`: HTML template with styling and Leaflet CSS/JS
- `Cargo.toml`: Project dependencies

## Understanding the Code

The key parts of the pane implementation with PaneContext integration:

```rust
// Define reactive z-index values (relative to Leaflet defaults: overlays=400, markers=600)
let background_z_index = 350.0;
let middle_z_index = 550.0;

// PaneStrategy::Context (default) - inherits parent pane context
<Pane name="background-pane" z_index=background_z_index>
    <Circle center=position!(51.505, -0.09) radius=400.0 color="blue">
        // Uses PaneStrategy::Context by default - inherits "background-pane"
        <Tooltip>"Background Circle using Context strategy"</Tooltip>
    </Circle>
    <Circle center=position!(51.505, -0.09) radius=300.0 color="darkblue">
        // Uses PaneStrategy::Custom to override the parent context
        <Tooltip pane_strategy=PaneStrategy::Custom(Signal::derive(|| "foreground-pane".to_string()))>
            "Background Circle using Custom strategy (overridden to foreground-pane)"
        </Tooltip>
    </Circle>
</Pane>

<Pane name="middle-pane" z_index=middle_z_index>
    <Marker position=position!(51.505, -0.09)>
        // Uses PaneStrategy::Context (default)
        <Popup>"Marker using Context strategy (middle-pane)"</Popup>
    </Marker>
    <Marker position=position!(51.503, -0.091)>
        // Uses PaneStrategy::Default to ignore context and use Leaflet's defaults
        <Popup pane_strategy=PaneStrategy::Default>
            "Marker using Default strategy (Leaflet's default popup pane)"
        </Popup>
    </Marker>
    <Marker position=position!(51.507, -0.088)>
        // Uses PaneStrategy::Custom with reactive signal
        <Popup pane_strategy=PaneStrategy::Custom(Signal::derive(|| "svg-renderer-pane".to_string()))>
            "Marker using Custom strategy (svg-renderer-pane)"
        </Popup>
    </Marker>
</Pane>

// Standalone components outside pane context
<Popup 
    position=position!(51.512, -0.085)
    pane_strategy=PaneStrategy::Default
>
    "Standalone Popup using Default strategy"
</Popup>

<Tooltip
    position=position!(51.498, -0.095)
    permanent=Signal::derive(|| true)
    pane_strategy=PaneStrategy::Custom(Signal::derive(|| "foreground-pane".to_string()))
>
    "Standalone Tooltip using Custom strategy"
</Tooltip>
```

## PaneStrategy Benefits

1. **Explicit Control**: Three clear strategies for pane handling - no ambiguity about behavior
2. **Flexible Options**: Choose the right strategy for each use case:
   - `Context`: Automatic inheritance (most common)
   - `Custom`: Explicit pane control with reactive capabilities
   - `Default`: Leaflet's standard behavior when you don't want custom panes
3. **Reactive Support**: Custom strategy accepts reactive signals for dynamic pane switching
4. **Backward Compatible**: Default behavior maintains context inheritance like before

## Tips for Using PaneStrategy

1. **Choose the Right Strategy**:
   - Use `PaneStrategy::Context` (default) for most cases where you want automatic inheritance
   - Use `PaneStrategy::Custom(signal)` when you need explicit control or dynamic pane switching
   - Use `PaneStrategy::Default` when you want Leaflet's standard behavior regardless of context

2. **Pane Management**:
   - **Unique Names**: Each pane must have a unique name
   - **Z-Index Values**: Higher values appear on top. Consider Leaflet's defaults: tiles=200, overlays=400, shadows=500, markers=600, popups=700
   - **Conditional Rendering**: Use signals to control pane visibility

3. **Strategy Examples**:
   ```rust
   // Context strategy (default) - inherits parent pane
   <Tooltip>"Uses parent pane automatically"</Tooltip>
   
   // Custom strategy with static pane
   <Popup pane_strategy=PaneStrategy::Custom(Signal::derive(|| "my-pane".to_string()))>
       "Always uses 'my-pane'"
   </Popup>
   
   // Custom strategy with reactive pane
   let dynamic_pane = signal("pane1".to_string());
   <Tooltip pane_strategy=PaneStrategy::Custom(dynamic_pane.into())>
       "Pane can change reactively"
   </Tooltip>
   
   // Default strategy - ignores all context
   <Popup pane_strategy=PaneStrategy::Default>
       "Uses Leaflet's default popup pane"
   </Popup>
   ```

4. **Performance**: Only create panes when needed to avoid unnecessary DOM elements
5. **Testing Overlap**: Position elements at the same coordinates to easily verify z-index behavior
6. **Custom Renderers**: Specify `renderer=RendererType::Svg` or `renderer=RendererType::Canvas` for vector layers