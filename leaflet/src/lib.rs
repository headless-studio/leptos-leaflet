mod control;
mod crs;
mod div_icon;
mod div_overlay;
mod event;
mod evented;
mod feature_group;
mod geo_json;
mod grid_layer;
mod handler;
mod icon;
mod lat_lng;
mod latlng_bounds;
mod layer;
mod layer_control;
mod layer_group;
mod map;
mod marker;
mod point;
mod popup;
mod raster;
mod shapes;
mod tooltip;

use js_sys::Array;

pub use control::Control;
pub use crs::Crs;
pub use div_icon::{DivIcon, DivIconOptions};
pub use div_overlay::DivOverlay;
pub use event::Event;
pub use evented::{
    DragEvents, Evented, EventedHandle, LayerEvents, MouseEvents, MoveEvents, PopupEvents,
    TooltipEvents,
};
pub use feature_group::FeatureGroup;
pub use geo_json::GeoJSON;
pub use grid_layer::{GridLayer, GridLayerOptions};
pub use handler::Handler;
pub use icon::{Icon, IconOptions};
pub use lat_lng::LatLng;
pub use latlng_bounds::LatLngBounds;
pub use layer::Layer;
pub use layer_group::LayerGroup;
pub use map::{
    DragEndEvent, ErrorEvent, LocateOptions, LocationEvent, Map, MapOptions, MouseEvent,
    PopupEvent, TooltipEvent,
};
pub use marker::{Marker, MarkerOptions};
pub use point::Point;
pub use popup::{Popup, PopupOptions};
pub use raster::{
    ImageOverlay, ImageOverlayOptions, TileLayer, TileLayerOptions, TileLayerWms,
    TileLayerWmsOptions, VideoOverlay, VideoOverlayOptions,
};
pub use shapes::{
    Circle, CircleMarker, CircleOptions, Path, PathOptions, Polygon, Polyline, PolylineOptions,
    Rectangle,
};
pub use tooltip::{Tooltip, TooltipOptions};

#[macro_export]
macro_rules! object_property_set {
    ($a:ident, $b:ty) => {
        pub fn $a(&mut self, val: $b) -> &mut Self {
            let r = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($a)),
                &wasm_bindgen::JsValue::from(val),
            );
            let _ = r;
            self
        }
    };
    ($a:ident, $b:ident, $c:ty) => {
        pub fn $a(&mut self, val: $c) -> &mut Self {
            let r = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($b)),
                &wasm_bindgen::JsValue::from(val),
            );
            let _ = r;
            self
        }
    };
}

#[macro_export]
macro_rules! object_property_set_with {
    ($a:ident, $b:ident, $c:expr) => {
        pub fn $a(&mut self) -> &mut Self {
            let r = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($b)),
                &wasm_bindgen::JsValue::from($c),
            );
            let _ = r;
            self
        }
    };
}

#[macro_export]
macro_rules! object_constructor {
    () => {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_mut)]
            let mut r = JsCast::unchecked_into(Object::new());
            r
        }
    };
}

#[allow(clippy::from_over_into)]
impl Into<LatLngBounds> for (LatLng, LatLng) {
    fn into(self) -> LatLngBounds {
        LatLngBounds::new(&self.0, &self.1)
    }
}

pub fn to_lat_lng_array<T: Into<LatLng> + Clone>(lat_lngs: &[T]) -> Array {
    let array = Array::new();
    for lat_lng in lat_lngs.iter().cloned() {
        array.push(&lat_lng.into());
    }
    array
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Point {
        Point::new(x as f64, y as f64)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Point {
        Point::new(x, y)
    }
}
