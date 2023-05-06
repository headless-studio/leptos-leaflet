use crate::Event;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone, PartialEq)]
    pub type Evented;

    /// Creates a new Evented object.
    ///
    /// [`on`](https://leafletjs.com/reference.html#evented-on)
    #[wasm_bindgen(method)]
    pub fn on(this: &Evented, kind: &str, handler: &JsValue) -> Evented;

    /// Removes an event listener.
    ///
    /// [`off`](https://leafletjs.com/reference.html#evented-off)
    #[wasm_bindgen(method)]
    pub fn off(this: &Evented, kind: &str, handler: &JsValue) -> Evented;

    /// Removes all event listeners.
    ///
    /// [`off`](https://leafletjs.com/reference.html#evented-off)
    #[wasm_bindgen(method, js_name = off)]
    pub fn offAll(this: &Evented) -> Evented;

    /// Emits an event.
    ///
    /// [`fire`](https://leafletjs.com/reference.html#evented-fire)
    #[wasm_bindgen(method)]
    pub fn fire(this: &Evented, kind: &str, data: &Object, propagate: Option<bool>) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`listens`](https://leafletjs.com/reference.html#evented-listens)
    #[wasm_bindgen(method)]
    pub fn listens(this: &Evented, kind: &str, propagate: Option<bool>) -> bool;

    /// Returns true if the event has listeners.
    ///
    /// [`once`](https://leafletjs.com/reference.html#evented-once)
    #[wasm_bindgen(method)]
    pub fn once(this: &Evented, kind: &str, handler: &JsValue) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`addEventParent`](https://leafletjs.com/reference.html#evented-addeventparent)
    #[wasm_bindgen(method)]
    pub fn addEventParent(this: &Evented, other: &Evented) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// ['removeEventParent'](https://leafletjs.com/reference.html#evented-removeeventparent)
    #[wasm_bindgen(method)]
    pub fn removeEventParent(this: &Evented, other: &Evented) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`addEventListener`](https://leafletjs.com/reference.html#evented-addeventlistener)
    #[wasm_bindgen(method)]
    pub fn addEventListener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`removeEventListener`](https://leafletjs.com/reference.html#evented-removeeventlistener)
    #[wasm_bindgen(method)]
    pub fn removeEventListener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    /// Clears all event listeners.
    ///
    /// [`clearAllEventListeners`](https://leafletjs.com/reference.html#evented-cleareventlisteners)
    #[wasm_bindgen(method)]
    pub fn clearAllEventListeners(this: &Evented) -> Evented;

    /// Adds a one time event listener.
    ///
    /// [`addOneTimeEventListener`](https://leafletjs.com/reference.html#evented-addonetimeeventlistener)
    #[wasm_bindgen(method)]
    pub fn addOneTimeEventListener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    /// Fires an event.
    ///
    /// [`fireEvent`](https://leafletjs.com/reference.html#evented-fireevent)
    #[wasm_bindgen(method)]
    pub fn fireEvent(this: &Evented, kind: &str, data: &Object, propagate: Option<bool>)
        -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`hasEventListeners`](https://leafletjs.com/reference.html#evented-haseventlisteners)
    #[wasm_bindgen(method)]
    pub fn hasEventListeners(this: &Evented, kind: &str, propagate: Option<bool>) -> bool;
}
