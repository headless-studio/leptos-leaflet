# Pane Demo Example

This example demonstrates how to use the `Pane` component in leptos-leaflet to create custom map panes with different z-index values for controlling layer stacking order.

## What This Example Shows

- **Three Custom Panes**: Background, Middle, and Foreground panes with z-index values of 350, 550, and 650 respectively
- **Interactive Controls**: Checkboxes to toggle each pane on/off
- **Z-Index Demonstration**: Visual demonstration of how z-index affects layer stacking relative to Leaflet's default panes
- **Mixed Content**: Different map elements (circles, markers, polygons) in different panes with overlapping geometry

## Pane Configuration

### Background Pane (z-index: 350)
- Contains blue circles with light blue fill
- Below Leaflet's default overlay pane (400), appears behind most elements
- Can be toggled with the first checkbox

### Middle Pane (z-index: 550)
- Contains red markers with custom styling
- Between Leaflet's shadow pane (500) and marker pane (600)
- Can be toggled with the second checkbox

### Foreground Pane (z-index: 650)
- Contains green polygons with light green fill
- Above Leaflet's default marker pane (600), appears on top of most elements
- Can be toggled with the third checkbox

### Default Elements
- An orange circle using Leaflet's default overlay pane (z-index: 400)
- A default marker using Leaflet's default marker pane (z-index: 600)
- All elements are positioned at the same center point to clearly demonstrate z-index layering

## Key Features Demonstrated

1. **Custom Z-Index**: Each pane has a different z-index to control stacking order relative to Leaflet's defaults
2. **Dynamic Visibility**: Panes can be toggled on/off using reactive signals
3. **Pane Context**: Child components automatically use their parent pane
4. **Mixed Layer Types**: Different Leaflet layer types work within panes
5. **Geometric Overlap**: All elements overlap at the map center to clearly show layering effects

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

The key parts of the pane implementation:

```rust
// Define reactive z-index values (relative to Leaflet defaults: overlays=400, markers=600)
let background_z_index = Signal::derive(move || {
    if show_background_pane.get() { 350.0 } else { 0.0 }
});

// Create a pane with custom z-index
<Pane name="background-pane" z_index=background_z_index>
    <Circle center=position!(51.505, -0.09) radius=400.0 color="blue">
        <Tooltip>"Background Circle (z-index: 350)"</Tooltip>
    </Circle>
</Pane>
```

## Tips for Using Panes

1. **Unique Names**: Each pane must have a unique name
2. **Z-Index Values**: Higher values appear on top. Consider Leaflet's defaults: tiles=200, overlays=400, shadows=500, markers=600, popups=700
3. **Conditional Rendering**: Use signals to control pane visibility
4. **Child Context**: Components inside a pane automatically use that pane
5. **Performance**: Only create panes when needed to avoid unnecessary DOM elements
6. **Testing Overlap**: Position elements at the same coordinates to easily verify z-index behavior