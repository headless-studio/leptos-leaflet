use wasm_bindgen::prelude::*;

use crate::{LatLngBounds, Polygon, PolylineOptions};

#[wasm_bindgen]
extern "C" {
    // Rectangle

    #[derive(Debug)]
    #[wasm_bindgen(extends = Polygon)]
    pub type Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(bounds: &LatLngBounds) -> Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(bounds: &LatLngBounds, options: &PolylineOptions) -> Rectangle;

    #[wasm_bindgen(method)]
    pub fn setBounds(this: &Rectangle, bounds: &LatLngBounds) -> Rectangle;
}
