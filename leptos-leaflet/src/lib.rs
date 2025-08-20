//! # leptos-leaflet
//!
//! This crate provides a set of components and utilities to work with the Leaflet library in Leptos.
//!
//! ## Components
//!
//! - [`MapContainer`](crate::MapContainer): A container for the Leaflet map. Where all the other components are added.
//! - [`Circle`](crate::Circle): A circle overlay that represents a circle on the map.
//! - [`Control`](crate::Control): A control that represents a control on the map.
//! - [`ImageOverlay`](crate::ImageOverlay): An image overlay that represents an image on the map.
//! - [`Marker`](crate::Marker): A marker overlay that represents a marker on the map.
//! - [`Pane`](crate::Pane): A custom map pane for organizing layers with custom z-index ordering.
//! - [`Polygon`](crate::Polygon): A polygon overlay that represents a polygon on the map.
//! - [`Polyline`](crate::Polyline): A polyline overlay that represents a polyline on the map.
//! - [`Popup`](crate::Popup): A popup overlay that represents a popup on the map.
//! - [`QuadTileLayer`](crate::QuadTileLayer): A tile layer that uses quadkey-based URLs instead of x/y/z coordinates.
//! - [`TileLayer`](crate::TileLayer): A tile layer that represents a tile layer on the map.
//! - [`TileLayerWms`](crate::TileLayerWms): A tile layer that represents a tile layer on the map.
//! - [`Tooltip`](crate::Tooltip): A tooltip overlay that represents a tooltip on the map.
//! - [`VideoOverlay`](crate::VideoOverlay): A video overlay that represents a video on the map.
//! - [`Zoom`](crate::Zoom): A zoom control that represents a zoom control on the map.
//!
//! ## Utilities
//!
//! - [`IntoLatLng`](crate::IntoLatLng): A trait to convert types into `leaflet::LatLng` instances.
//! - [`LeafletMapContext`](crate::LeafletMapContext): A context struct for the Leaflet map.
//! - [`Position`](crate::Position): A struct to represent a position on the map.
//!
//! ## Example
//!
//! ```rust
//! use std::time::Duration;
//!
//! use leptos::prelude::*;
//! use leptos_leaflet::prelude::*;
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     let (marker_position, set_marker_position) = create_signal(Position::new(51.49, -0.08));
//!
//!     create_effect(move |_| {
//!         set_interval_with_handle(
//!             move || {
//!                 set_marker_position.update(|pos| {
//!                     pos.lat += 0.001;
//!                     pos.lng += 0.001;
//!                 });
//!             },
//!             Duration::from_millis(200),
//!         )
//!         .ok()
//!     });
//!
//!     view! {
//!           <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
//!               <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
//!               <Marker position=marker_position >
//!                   <Popup>
//!                       <strong>{"A pretty CSS3 popup"}</strong>
//!                   </Popup>
//!               </Marker>
//!                 <Marker position=(51.5, -0.065) draggable=true >
//!                   <Popup>
//!                       <strong>{"A pretty CSS3 popup"}</strong>
//!                   </Popup>
//!               </Marker>
//!               <Tooltip position=(51.5, -0.06) permanent=true direction="top">
//!                   <strong>{"And a tooltip"}</strong>
//!               </Tooltip>
//!               <Polyline positions=positions(&[(51.505, -0.09), (51.51, -0.1), (51.51, -0.12)])/>
//!               <Polygon color="purple" positions=positions(&[ (51.515, -0.09), (51.52, -0.1), (51.52, -0.12)]) >
//!                 <Tooltip sticky=true direction="top">
//!                     <strong>{"I'm a polygon"}</strong>
//!                 </Tooltip>
//!             </Polygon>
//!             <Circle center=(51.505, -0.09) color="blue" radius=200.0>
//!                 <Tooltip sticky=true>{"I'm a circle"}</Tooltip>
//!             </Circle>
//!         </MapContainer>
//!     }
//! }
//! ```
mod components;
mod core;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::core::*;
    pub use crate::position;
}

/// Leaflet re-exports
pub use leaflet;

use paste::paste;
