use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{object_construtor, object_property_set, CircleMarker, LatLng, LatLngBounds};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = CircleOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type CircleOptions;

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = CircleMarker)]
    pub type Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &JsValue) -> Circle;

    /// [`setRadius`](https://leafletjs.com/reference-1.7.1.html#circle-setradius)
    #[wasm_bindgen(method)]
    pub fn setRadius(this: &Circle, radius: f64);

    /// [`getRadius`](https://leafletjs.com/reference-1.7.1.html#circle-getradius)
    #[wasm_bindgen(method)]
    pub fn getRadius(this: &Circle) -> f64;

    /// [`getBounds`](https://leafletjs.com/reference.html#circle-getbounds)
    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Circle) -> LatLngBounds;

}

impl CircleOptions {
    object_construtor!();
    object_property_set!(radius, f64);
}
