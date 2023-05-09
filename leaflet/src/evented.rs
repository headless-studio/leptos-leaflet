use crate::map::{DragEndEvent, TooltipEvent};
use crate::{Circle, Event, MouseEvent, PopupEvent};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone, PartialEq)]
    pub type Evented;

    /// Creates a new Evented object.
    ///
    /// [`on`](https://leafletjs.com/reference.html#evented-on)
    #[wasm_bindgen(method, js_name = on)]
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

pub trait LeafletEventHandler {
    fn on(&self, event: &str, callback: &JsValue);
}

pub trait MoveEvents
where
    Self: LeafletEventHandler,
{
    fn on_move(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("move", &closure.into_js_value());
    }
}

pub trait MouseEvents
where
    Self: LeafletEventHandler,
{
    fn on_click(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("click", &closure.into_js_value());
    }

    fn on_double_click(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("dblclick", &closure.into_js_value());
    }

    fn on_mouse_down(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mousedown", &closure.into_js_value());
    }

    fn on_mouse_up(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseup", &closure.into_js_value());
    }

    fn on_mouse_over(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseover", &closure.into_js_value());
    }

    fn on_mouse_out(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseout", &closure.into_js_value());
    }

    fn on_context_menu(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("contextmenu", &closure.into_js_value());
    }
}

pub trait DragEvents
where
    Self: LeafletEventHandler,
{
    fn on_drag_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("dragstart", &closure.into_js_value());
    }

    fn on_move_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("movestart", &closure.into_js_value());
    }

    fn on_drag(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("drag", &closure.into_js_value());
    }

    fn on_drag_end(&self, callback: Box<dyn Fn(DragEndEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("dragend", &closure.into_js_value());
    }

    fn on_move_end(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("moveend", &closure.into_js_value());
    }
}

pub trait LayerEvents
where
    Self: LeafletEventHandler,
{
    fn on_add(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("add", &closure.into_js_value());
    }

    fn on_remove(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("remove", &closure.into_js_value());
    }
}

pub trait PopupEvents
where
    Self: LeafletEventHandler,
{
    fn on_popup_open(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupopen", &closure.into_js_value());
    }

    fn on_popup_close(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupclose", &closure.into_js_value());
    }
}

pub trait TooltipEvents
where
    Self: LeafletEventHandler,
{
    fn on_tooltip_open(&self, callback: Box<dyn Fn(TooltipEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("tooltipopen", &closure.into_js_value());
    }

    fn on_tooltip_close(&self, callback: Box<dyn Fn(TooltipEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("tooltipclose", &closure.into_js_value());
    }
}
