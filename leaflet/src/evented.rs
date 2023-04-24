use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Evented;

    #[wasm_bindgen(method)]
    pub fn on(this: &Evented, kind: &str, handler: &Closure<dyn Fn(JsValue)>) -> Evented;

    #[wasm_bindgen(method)]
    pub fn off(this: &Evented, kind: &str, handler: &Closure<dyn Fn(JsValue)>) -> Evented;

    #[wasm_bindgen(method, js_name = off)]
    pub fn off_all(this: &Evented) -> Evented;

    #[wasm_bindgen(method)]
    pub fn fire(this: &Evented, kind: &str, data: &Object, propagate: Option<bool>) -> Evented;

    #[wasm_bindgen(method)]
    pub fn listens(this: &Evented, kind: &str, propagate: Option<bool>) -> bool;

    #[wasm_bindgen(method)]
    pub fn once(this: &Evented, kind: &str, handler: &Closure<dyn Fn(JsValue)>) -> Evented;

    #[wasm_bindgen(method)]
    pub fn addEventParent(this: &Evented, other: &Evented) -> Evented;

    #[wasm_bindgen(method)]
    pub fn removeEventParent(this: &Evented, other: &Evented) -> Evented;

    #[wasm_bindgen(method)]
    pub fn addEventListener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    #[wasm_bindgen(method)]
    pub fn removeEventListener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    #[wasm_bindgen(method)]
    pub fn clearAllEventListeners(this: &Evented) -> Evented;

    #[wasm_bindgen(method)]
    pub fn addOneTimeEventListener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    #[wasm_bindgen(method)]
    pub fn fireEvent(this: &Evented, kind: &str, data: &Object, propagate: Option<bool>)
        -> Evented;

    #[wasm_bindgen(method)]
    pub fn hasEventListeners(this: &Evented, kind: &str, propagate: Option<bool>) -> bool;
}
