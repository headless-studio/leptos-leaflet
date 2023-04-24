use wasm_bindgen::prelude::*;

use crate::{LatLng, Polyline, PolylineOptions};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Polyline)]
    pub type Polygon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlngs: Vec<LatLng>) -> Polygon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlngs: Vec<LatLng>, options: &PolylineOptions) -> Polygon;
}
