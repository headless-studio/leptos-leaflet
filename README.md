# leptos-leaflet
![license: MIT](https://img.shields.io/crates/l/syn-rsx.svg)

Leaflet components for Leptos. This aims to target the funcionality of React-Leaflet. 

For now only a few of the components are ported, and events must be set in the Signal `map` object setted by the MapContainer when leaflet is inited.

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
- Better way to handle events
- Add all the options as signals for all components
- Implement missing components
