# leptos-leaflet
[![crates.io](https://img.shields.io/crates/v/leptos-leaflet.svg)](https://crates.io/crates/leptos-leaflet)
![license: MIT](https://img.shields.io/crates/l/syn-rsx.svg)

Leaflet components for Leptos. This aims to target the functionality of React-Leaflet. 

For now only a few of the components are ported, and events must be set in the Signal `map` object set by the MapContainer when leaflet is inited.

NOTE: Current version support leptos 0.5.0-beta, that removes all Scope usages from signals and effects.

## Features
- CSR/HYDRATE/SSR support

### Components
- MapContainer
- TileLayer
- ImageOverlay
- VideoOverlay
- Marker
- Polygon
- Polyline
- Circle
- Tooltip
- Popup

## TODO
- Implement missing components
