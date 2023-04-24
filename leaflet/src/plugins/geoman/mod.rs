use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use crate::{object_construtor, object_property_set, Map};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone)]
    pub type GeomanControls;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone)]
    pub type Geoman;

    #[wasm_bindgen(method)]
    pub fn addControls(this: &Geoman, options: &GeomanControls);

    #[wasm_bindgen(method)]
    pub fn removeControls(this: &Geoman);

    #[wasm_bindgen(method)]
    pub fn toggleControls(this: &Geoman);

    #[wasm_bindgen(method)]
    pub fn controlsVisible(this: &Geoman);

    #[wasm_bindgen(method)]
    pub fn enableDraw(this: &Geoman, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn disableDraw(this: &Geoman);

    #[wasm_bindgen(method)]
    pub fn globalDrawModeEnabled(this: &Geoman) -> bool;

    #[wasm_bindgen(method)]
    pub fn enableGlobalDragMode(this: &Geoman) -> bool;

    #[wasm_bindgen(method)]
    pub fn enableLayerDrag(this: &Geoman) -> bool;

    #[wasm_bindgen(method)]
    pub fn setPathOptions(this: &Geoman, options: &JsValue, options_modifier: &JsValue);

    #[wasm_bindgen(method)]
    pub fn setGlobalOptions(this: &Geoman, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn applyGlobalOptions(this: &Geoman);

    #[wasm_bindgen(method)]
    pub fn getGlobalOptions(this: &Geoman) -> Object;

    #[wasm_bindgen(method)]
    pub fn getGeomanLayers(this: &Geoman, all: bool) -> Array;

    #[wasm_bindgen(method, js_namespace=Draw)]
    pub fn getShapes(this: &Geoman) -> Array;

    #[wasm_bindgen(method, js_namespace=Draw)]
    pub fn getActiveShape(this: &Geoman) -> String;
}

impl Map {
    pub fn pm(&self) -> Geoman {
        let pm = js_sys::Reflect::get(self, &JsValue::from("pm")).expect("geoman installed");
        pm.into()
    }
}

impl GeomanControls {
    object_construtor!();
    object_property_set!(position, &str);
    object_property_set!(positions, &JsValue);
    object_property_set!(draw_marker, drawMarker, bool);
    object_property_set!(draw_circle_marker, drawCircleMarker, bool);
    object_property_set!(draw_polyline, drawPolyline, bool);
    object_property_set!(draw_rectangle, drawRectangle, bool);
    object_property_set!(draw_polygon, drawPolygon, bool);
    object_property_set!(draw_circle, drawCircle, bool);
    object_property_set!(draw_text, drawText, bool);
    object_property_set!(edit_mode, editMode, bool);
    object_property_set!(drag_mode, dragMode, bool);
    object_property_set!(cut_polygon, cutPolygon, bool);
    object_property_set!(removal_mode, removalMode, bool);
    object_property_set!(rotate_mode, rotateMode, bool);
    object_property_set!(one_block, oneBlock, bool);
    object_property_set!(draw_controls, drawControls, bool);
    object_property_set!(edit_controls, editControls, bool);
    object_property_set!(custom_controls, customControls, bool);
    object_property_set!(options_controls, optionsControls, bool);
    object_property_set!(pinning_option, pinningOption, bool);
    object_property_set!(snapping_option, snappingOption, bool);
    object_property_set!(split_mode, splitMode, bool);
    object_property_set!(scale_mode, scaleMode, bool);
    object_property_set!(auto_tracing_option, autoTracingOption, bool);
}

impl Default for GeomanControls {
    fn default() -> Self {
        GeomanControls::new()
    }
}
