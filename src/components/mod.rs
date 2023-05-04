mod circle;
mod context;
mod image_overlay;
mod map_container;
mod map_events;
mod marker;
mod path_options;
mod polygon;
mod polyline;
mod popup;
mod tile_layer;
mod tooltip;
mod video_overlay;

use leaflet::LatLng;

pub use circle::Circle;
pub use context::*;
pub use leaflet::{CircleOptions, PathOptions, PolylineOptions};
pub use map_container::{LeafletMap, MapContainer};
pub use map_events::MapEvents;
pub use marker::Marker;
pub use path_options::*;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use popup::Popup;
pub use tile_layer::TileLayer;
pub use tooltip::Tooltip;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

impl Position {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

impl From<Position> for LatLng {
    fn from(value: Position) -> Self {
        LatLng::new(value.lat, value.lng)
    }
}

impl From<Position> for (f64, f64) {
    fn from(value: Position) -> Self {
        (value.lat, value.lng)
    }
}

impl From<Position> for [f64; 2] {
    fn from(value: Position) -> Self {
        [value.lat, value.lng]
    }
}

impl From<LineJoin> for String {
    fn from(value: LineJoin) -> Self {
        format!("{}", value)
    }
}

impl From<LineCap> for String {
    fn from(value: LineCap) -> Self {
        format!("{}", value)
    }
}

impl From<FillRule> for String {
    fn from(value: FillRule) -> Self {
        format!("{}", value)
    }
}
