use crate::Map;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(js_namespace = L, js_name = Control)]
    pub type Control;

    #[wasm_bindgen(method)]
    pub fn getPosition(this: &Control) -> String;

    #[wasm_bindgen(method)]
    pub fn setPosition(this: &Control, position: &str) -> Control;

    #[wasm_bindgen(method)]
    pub fn getContainer(this: &Control) -> HtmlElement;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Control, map: &Map) -> Control;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Control) -> Control;
}
