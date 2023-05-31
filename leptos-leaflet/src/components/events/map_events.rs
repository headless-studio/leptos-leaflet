use crate::leaflet_event;
use leaflet::{ErrorEvent, Event, LocationEvent, Map, MouseEvent, PopupEvent};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct MapEvents {
    inner: Rc<RefCell<InnerMapEvents>>,
}

#[derive(Default)]
struct InnerMapEvents {
    location_found: Option<Box<dyn Fn(LocationEvent)>>,
    location_error: Option<Box<dyn Fn(ErrorEvent)>>,
    load: Option<Box<dyn Fn(Event)>>,
    unload: Option<Box<dyn Fn(Event)>>,
    resize: Option<Box<dyn Fn(Event)>>,
    zoom_start: Option<Box<dyn Fn(Event)>>,
    zoom_end: Option<Box<dyn Fn(Event)>>,
    move_: Option<Box<dyn Fn(Event)>>,
    move_start: Option<Box<dyn Fn(Event)>>,
    move_end: Option<Box<dyn Fn(Event)>>,
    mouse_click: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_double_click: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_context_menu: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_move: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_over: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_out: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_down: Option<Box<dyn Fn(MouseEvent)>>,
    mouse_up: Option<Box<dyn Fn(MouseEvent)>>,
}

impl MapEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, map: &Map) {
        if let Some(location_found) = self.inner.borrow_mut().location_found.take() {
            map.on_location_found(location_found);
        }
        if let Some(location_error) = self.inner.borrow_mut().location_error.take() {
            map.on_location_error(location_error);
        }
        if let Some(load) = self.inner.borrow_mut().load.take() {
            map.on_load(load);
        }
        if let Some(unload) = self.inner.borrow_mut().unload.take() {
            map.on_unload(unload);
        }
        if let Some(resize) = self.inner.borrow_mut().resize.take() {
            map.on_resize(resize);
        }
        if let Some(zoom) = self.inner.borrow_mut().zoom.take() {
            map.on_zoom(zoom);
        }
        if let Some(zoom_start) = self.inner.borrow_mut().zoom_start.take() {
            map.on_zoom_start(zoom_start);
        }
        if let Some(zoom_end) = self.inner.borrow_mut().zoom_end.take() {
            map.on_zoom_end(zoom_end);
        }
        if let Some(move_start) = self.inner.borrow_mut().move_start.take() {
            map.on_move_start(move_start);
        }
        if let Some(move_end) = self.inner.borrow_mut().move_end.take() {
            map.on_move_end(move_end);
        }
        if let Some(move_) = self.inner.borrow_mut().move_.take() {
            map.on_move(move_);
        }
        if let Some(mouse_click) = self.inner.borrow_mut().mouse_click.take() {
            map.on_mouse_click(mouse_click);
        }
        if let Some(mouse_double_click) = self.inner.borrow_mut().mouse_double_click.take() {
            map.on_mouse_double_click(mouse_double_click);
        }
        if let Some(mouse_context_menu) = self.inner.borrow_mut().mouse_context_menu.take() {
            map.on_mouse_context_menu(mouse_context_menu);
        }
        if let Some(mouse_move) = self.inner.borrow_mut().mouse_move.take() {
            map.on_mouse_move(mouse_move);
        }
        if let Some(mouse_over) = self.inner.borrow_mut().mouse_over.take() {
            map.on_mouse_over(mouse_over);
        }
        if let Some(mouse_out) = self.inner.borrow_mut().mouse_out.take() {
            map.on_mouse_out(mouse_out);
        }
        if let Some(mouse_down) = self.inner.borrow_mut().mouse_down.take() {
            map.on_mouse_down(mouse_down);
        }
        if let Some(mouse_up) = self.inner.borrow_mut().mouse_up.take() {
            map.on_mouse_up(mouse_up);
        }
    }
}

leaflet_event!(MapEvents, location_found, LocationEvent);
leaflet_event!(MapEvents, location_error, ErrorEvent);
leaflet_event!(MapEvents, load, Event);
leaflet_event!(MapEvents, unload, Event);
leaflet_event!(MapEvents, resize, Event);
leaflet_event!(MapEvents, zoom, Event);
