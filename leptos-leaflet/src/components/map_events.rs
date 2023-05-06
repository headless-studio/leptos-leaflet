use leaflet::{ErrorEvent, Event, LocationEvent, PopupEvent};
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
    popup_open: Option<Box<dyn Fn(PopupEvent)>>,
    popup_close: Option<Box<dyn Fn(PopupEvent)>>,
    zoom: Option<Box<dyn Fn(Event)>>,
}

macro_rules! leaflet_event {
    ($with:ident, $take:ident, $e:ident, $t:ty) => {
        impl MapEvents {
            /// Set ups event $n
            pub fn $with(self, callback: impl Fn($t) + 'static) -> Self {
                self.inner.borrow_mut().$e = Some(Box::new(callback));
                self
            }

            /// Takes the callback for event $n from the struture
            pub fn $take(&self) -> Option<Box<dyn Fn($t)>> {
                self.inner.borrow_mut().$e.take()
            }
        }
    };
}

impl MapEvents {
    pub fn new() -> Self {
        Self::default()
    }
}

leaflet_event!(with_zoom, take_zoom, zoom, Event);
leaflet_event!(
    with_location_found,
    take_location_found,
    location_found,
    LocationEvent
);
leaflet_event!(
    with_location_error,
    take_location_error,
    location_error,
    ErrorEvent
);
leaflet_event!(with_popup_open, take_popup_open, popup_open, PopupEvent);
leaflet_event!(with_popup_close, take_popup_close, popup_close, PopupEvent);
leaflet_event!(with_load, take_load, load, Event);
leaflet_event!(with_unload, take_unload, unload, Event);
leaflet_event!(with_resize, take_resize, resize, Event);
