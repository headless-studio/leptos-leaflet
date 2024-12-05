# leptos-leaflet
[![crates.io](https://img.shields.io/crates/v/leptos-leaflet.svg)](https://crates.io/crates/leptos-leaflet)
[![docs.rs](https://docs.rs/leptos-leaflet/badge.svg)](https://docs.rs/leptos-leaflet)
[![license: MIT](https://img.shields.io/crates/l/syn-rsx.svg)](https://github.com/headless-studio/leptos-leaflet/LICENSE)

Leaflet components for Leptos. This aims to target the functionality of React-Leaflet. 

For now only a few of the components are ported, and events must be set in the Signal `map` object set by the MapContainer when leaflet is inited.

NOTE: Current version support leptos 0.6.x, that removes all Scope usages from signals and effects.

Support for leptos 0.7.x exists in the `leptos-0.7` branch. When Leptos 0.7 is released, this branch will be merged into main.

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

## Compatibility

| Crate version | Leptos version |
|---------------|----------------|
| 0.9.x         | 0.7.x          |
| 0.8.x         | 0.6.x          |