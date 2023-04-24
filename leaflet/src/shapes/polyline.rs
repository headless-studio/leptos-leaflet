use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use crate::{object_construtor, object_property_set, LatLng, LatLngBounds, Path, Point};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Object, js_name = PolylineOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type PolylineOptions;

    #[wasm_bindgen(extends = Path)]
    #[derive(Debug, Clone)]
    pub type Polyline;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> Polyline;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(options: &PolylineOptions) -> Polyline;

    #[wasm_bindgen(method, js_name = toGeoJSON)]
    pub fn toGeoJSON(this: &Polyline, precision: f64) -> Object;

    #[wasm_bindgen(method, js_name = getLatLngs)]
    pub fn getLatLngs(this: &Polyline) -> Array;

    #[wasm_bindgen(method, js_name = setLatLngs)]
    pub fn setLatLngs(this: &Polyline, lat_lngs: &Array) -> Polyline;

    #[wasm_bindgen(method, js_name = isEmpty)]
    pub fn isEmpty(this: &Polyline) -> bool;

    #[wasm_bindgen(method, js_name = closestLayerPoint)]
    pub fn closestLayerPoint(this: &Polyline, point: &Point) -> Point;

    #[wasm_bindgen(method, js_name = getCenter)]
    pub fn getCenter(this: &Polyline) -> LatLng;

    #[wasm_bindgen(method, js_name = getBounds)]
    pub fn getBounds(this: &Polyline) -> LatLngBounds;

    #[wasm_bindgen(method,js_name = addLatLng)]
    pub fn addLatLng(this: &Polyline, lat_lng: &LatLng) -> Polyline;
}

impl PolylineOptions {
    object_construtor!();
    object_property_set!(smooth_factor, smoothFactor, f64);
    object_property_set!(no_clip, noClip, bool);
}
