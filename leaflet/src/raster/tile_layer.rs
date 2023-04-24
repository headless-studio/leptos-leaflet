use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{GridLayer, LatLng, Map};

#[wasm_bindgen]
extern "C" {

    # [wasm_bindgen (extends = Object , js_name = TileLayerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[wasm_bindgen(extends = GridLayer)]
    pub type TileLayerOptions;

    #[derive(Debug, Clone, PartialEq)]
    pub type TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(url_template: &str) -> TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_options(url_template: &str, options: &TileLayerOptions) -> TileLayer;

    #[wasm_bindgen(method, js_name = setUrl)]
    pub fn set_url(this: &TileLayer, url: &str, no_redraw: Option<bool>) -> TileLayer;

    #[wasm_bindgen(method, js_name = getTileUrl)]
    pub fn get_tile_url(this: &TileLayer, coords: &LatLng) -> String;

    #[wasm_bindgen(method, js_name = createTile)]
    pub fn create_tile(this: &TileLayer, lat_long: &LatLng) -> HtmlElement;

    #[wasm_bindgen(method, js_name = createTile)]
    pub fn create_tile_with_done(
        this: &TileLayer,
        lat_long: &LatLng,
        done: &Function,
    ) -> HtmlElement;

    #[wasm_bindgen(method, js_name = addTo)]
    pub fn add_to(this: &TileLayer, map: &Map);
}

impl TileLayerOptions {
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }

    pub fn min_zoom(&mut self, val: f64) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("minZoom"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn max_zoom(&mut self, val: f64) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("maxZoom"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn subdomains(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("subdomains"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn error_tile_url(&mut self, val: &str) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("errorTileUrl"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn zoom_offset(&mut self, val: f64) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("zoomOffset"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn tms(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(self.as_ref(), &JsValue::from("tms"), &JsValue::from(val));
        let _ = r;
        self
    }

    pub fn zoom_reverse(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("zoomReverse"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn detect_retina(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("detectRetina"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    pub fn cross_origin(&mut self, val: &JsValue) -> &mut Self {
        let r = Reflect::set(self.as_ref(), &JsValue::from("crossOrigin"), val);
        let _ = r;
        self
    }

    pub fn referrer_policy(&mut self, val: &JsValue) -> &mut Self {
        let r = Reflect::set(self.as_ref(), &JsValue::from("referrerPolicy"), val);
        let _ = r;
        self
    }
}

impl Default for TileLayerOptions {
    fn default() -> Self {
        TileLayerOptions::new()
    }
}
