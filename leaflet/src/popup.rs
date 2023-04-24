use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{LatLng, Layer, Map, object_construtor, object_property_set, Point};

#[wasm_bindgen]
extern "C" {
    // Popup

    # [wasm_bindgen (extends = Object , js_name = PopupOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type PopupOptions;

    /// [`Popup`](https://leafletjs.com/reference.html#popup)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Layer)]
    pub type Popup;

    /// [`L.popup`](https://leafletjs.com/reference.html#popup-l-popup)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &PopupOptions, layer: Option<&Layer>) -> Popup;

    /// [`L.popup`](/// [`L.popup`](https://leafletjs.com/reference.html#popup-l-popup))
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_lat_lng(lat_lng: &LatLng, options: &PopupOptions) -> Popup;

    /// [`getLatLng`](https://leafletjs.com/reference-1.7.1.html#popup-getlatlng)
    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &Popup) -> LatLng;

    /// [`setLatLng`](https://leafletjs.com/reference-1.7.1.html#popup-setlatlng)
    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Popup, latlng: &LatLng);

    /// [`getContent`](https://leafletjs.com/reference-1.7.1.html#popup-getcontent)
    #[wasm_bindgen(method)]
    pub fn getContent(this: &Popup) -> JsValue;

    /// [`setContent`](https://leafletjs.com/reference-1.7.1.html#popup-setcontent)
    #[wasm_bindgen(method)]
    pub fn setContent(this: &Popup, content: &JsValue);

    /// [`update`](https://leafletjs.com/reference-1.7.1.html#popup-update)
    #[wasm_bindgen(method)]
    pub fn update(this: &Popup);

    /// [`isOpen`](https://leafletjs.com/reference-1.7.1.html#popup-isopen)
    #[wasm_bindgen(method)]
    pub fn isOpen(this: &Popup) -> bool;

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#popup-bringtofront)
    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &Popup);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#popup-bringtoback)
    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &Popup);

    /// [`openOn`](https://leafletjs.com/reference-1.7.1.html#popup-openon)
    #[wasm_bindgen(method)]
    pub fn openOn(this: &Popup, map: &Map);
}

impl PopupOptions {
    object_construtor!();
    object_property_set!(pane, &str);
    object_property_set!(offset, Point);
    object_property_set!(min_width, minWidth, f64);
    object_property_set!(max_width, maxWidth, f64);
    object_property_set!(max_height, maxHeight, f64);
    object_property_set!(auto_pan, autoPan, bool);
    object_property_set!(auto_pan_padding_top_left, autoPanPaddingTopLeft, Point);
    object_property_set!(auto_pan_padding_bottom_right, autoPanPaddingBottomRight, Point);
    object_property_set!(auto_pan_padding, autoPanPadding, Point);
    object_property_set!(keep_in_view, keepInView, bool);
    object_property_set!(close_button, closeButton, bool);
    object_property_set!(auto_close, autoClose, bool);
    object_property_set!(close_on_escape_key, closeOnEscapeKey, bool);
    object_property_set!(close_on_click, closeOnClick, bool);
    object_property_set!(class_name, className, &str);
}

impl Default for PopupOptions {
    fn default() -> Self {
        Self::new()
    }
}