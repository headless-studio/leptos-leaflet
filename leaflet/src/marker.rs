use js_sys::Object;
use wasm_bindgen::prelude::*;
use crate::{Icon, LatLng, Layer, object_constructor, object_property_set, Point};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = MarkerOptions)]
    #[derive(Debug, Clone)]
    pub type MarkerOptions;
    
    // Marker
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Layer)]
    pub type Marker;

    // [`Marker`](https://leafletjs.com/reference-1.7.1.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(lat_lng: &LatLng) -> Marker;

    // [`Marker`](https://leafletjs.com/reference-1.7.1.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(lat_lng: &LatLng, options: &MarkerOptions) -> Marker;

    #[wasm_bindgen(method)]
    pub fn setIcon(this: &Marker, icon: &Icon);

    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &Marker) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Marker, latlng: &LatLng);

    #[wasm_bindgen(method)]
    pub fn on(this: &Marker, event_name: &str, handler: &JsValue);
}

impl MarkerOptions {
    object_constructor!();
    object_property_set!(icon, Icon);
    object_property_set!(keyboard, bool);
    object_property_set!(title, &str);
    object_property_set!(alt, &str);
    object_property_set!(z_index_offset, zIndexOffset, f64);
    object_property_set!(opacity, f64);
    object_property_set!(rise_on_hover, riseOnHover, bool);
    object_property_set!(rise_offset, riseOffset, f64);
    object_property_set!(pane, &str);
    object_property_set!(shadow_pane, shadowPane, &str);
    object_property_set!(bubbling_mouse_events, bubblingMouseEvents, bool);
    // Draggable marker options
    object_property_set!(draggable, bool);
    object_property_set!(auto_pan, autoPan, bool);
    object_property_set!(auto_pan_padding, autoPanPadding, Point);
    object_property_set!(auto_pan_speed, autoPanSpeed, f64);
    // Interactive layer
    object_property_set!(interactive, bool);
    // Layer
    object_property_set!(attribution, &str);
}

impl Default for MarkerOptions {
    fn default() -> Self {
        Self::new()
    }
}