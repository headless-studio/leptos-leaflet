use crate::leaflet_event;
use leaflet::{ErrorEvent, Event, LocationEvent, Map, PopupEvent};
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
    zoom: Option<Box<dyn Fn(Event)>>,
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
    }
}

leaflet_event!(
    MapEvents,
    location_found,
    LocationEvent
);
leaflet_event!(
    MapEvents,
    location_error,
    ErrorEvent
);
leaflet_event!(MapEvents, load, Event);
leaflet_event!(MapEvents, unload, Event);
leaflet_event!(MapEvents, resize, Event);
leaflet_event!(MapEvents, zoom, Event);
