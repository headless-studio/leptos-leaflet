use wasm_bindgen::prelude::*;
use crate::{Event, LocationEvent, Point, Popup};

use super::Map;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = Event, js_name = ErrorEvent)]
    pub type ErrorEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ErrorEvent;

    #[wasm_bindgen(method, getter, js_name = message)]
    pub fn message(this: &ErrorEvent) -> String;

    #[wasm_bindgen(method, setter, js_name = message)]
    pub fn setMessage(this: &ErrorEvent, value: &str);

    #[wasm_bindgen(method, getter, js_name = code)]
    pub fn code(this: &ErrorEvent) -> i32;

    #[wasm_bindgen(method, setter, js_name = code)]
    pub fn setCode(this: &ErrorEvent, value: i32);

    #[wasm_bindgen (extends = Event, js_name = ResetEvent)]
    pub type ResetEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ResetEvent;

    #[wasm_bindgen(method, getter, js_name = oldSize)]
    pub fn oldSize(this: &ResetEvent) -> Point;

    #[wasm_bindgen(method, setter, js_name = oldSize)]
    pub fn setOldSize(this: &ResetEvent, value: &Point);

    #[wasm_bindgen(method, getter, js_name = newSize)]
    pub fn newSize(this: &ResetEvent) -> Point;

    #[wasm_bindgen(method, setter, js_name = newSize)]
    pub fn setNewSize(this: &ResetEvent, value: &Point);

    #[wasm_bindgen(extends = Event, js_name = PopupEvent)]
    pub type PopupEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PopupEvent;

    #[wasm_bindgen(method, getter, js_name = popup)]
    pub fn popup(this: &PopupEvent) -> Popup;

    #[wasm_bindgen(method, setter, js_name = popup)]
    pub fn setPopup(this: &PopupEvent, value: &Popup);
}

impl Map {
    pub fn on_zoom_levels_change(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoomlevelschange", &closure.into_js_value());
    }

    pub fn on_resize(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("resize", &closure.into_js_value());
    }

    pub fn on_view_reset(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("viewreset", &closure.into_js_value());
    }

    pub fn on_load(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("load", &closure.into_js_value());
    }

    pub fn on_unload(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("unload", &closure.into_js_value());
    }

    pub fn on_zoom(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoom", &closure.into_js_value());
    }

    pub fn on_zoom_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoomstart", &closure.into_js_value());
    }

    pub fn on_zoom_end(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoomend", &closure.into_js_value());
    }

    pub fn on_move(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("move", &closure.into_js_value());
    }

    pub fn on_move_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("movestart", &closure.into_js_value());
    }

    pub fn on_move_end(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("moveend", &closure.into_js_value());
    }

    pub fn on_location_found(&self, callback: Box<dyn Fn(LocationEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("locationfound", &closure.into_js_value());
    }

    pub fn on_location_error(&self, callback: Box<dyn Fn(ErrorEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("locationerror", &closure.into_js_value());
    }

    pub fn on_popup_open(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupopen", &closure.into_js_value());
    }

    pub fn on_popup_close(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupclose", &closure.into_js_value());
    }
}