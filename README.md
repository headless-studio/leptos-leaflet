# leptos-leaflet
[![crates.io](https://img.shields.io/crates/v/leptos-leaflet.svg)](https://crates.io/crates/leptos-leaflet)
[![docs.rs](https://docs.rs/leptos-leaflet/badge.svg)](https://docs.rs/leptos-leaflet)
[![license: MIT](https://img.shields.io/crates/l/syn-rsx.svg)](https://github.com/headless-studio/leptos-leaflet/LICENSE)

Leaflet components for Leptos. This aims to target the functionality of React-Leaflet. 

For now only a few of the components are ported, and events must be set in the Signal `map` object set by the MapContainer when leaflet is inited.

## Features
- CSR/HYDRATE/SSR support

### Components
- MapContainer
- Control
- TileLayer
- TileLayerWms
- ImageOverlay
- VideoOverlay
- Marker
- Polygon
- Polyline
- Circle
- Tooltip
- Popup
- Pane

## Compatibility

| Crate version | Leptos version |
|---------------|----------------|
| 0.10.x        | 0.8.x          |
| 0.9.x         | 0.7.x          |
| 0.8.x         | 0.6.x          |

## Examples

The repository includes several examples demonstrating different features:

- **simple-map**: Basic map with markers, polygons, and tooltips
- **pane-demo**: Demonstrates the `Pane` component with custom z-index values and interactive controls
- **tilelayerwms**: Shows how to use WMS tile layers
- **quad-tile-test**: Testing quad tile functionality
- **leaflet-api**: Direct Leaflet API usage examples
- **with-server**: Server-side rendering example

To run an example:

```bash
cd examples/[example-name]
cargo build
# Serve the files with a local web server
```