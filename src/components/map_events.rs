use std::cell::RefCell;
use std::rc::Rc;
use leaflet::{ErrorEvent, Event, LocationEvent, PopupEvent};

#[derive(Clone, Default)]
pub struct MapEvents {
    inner: Rc<RefCell<InnerMapEvents>>
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
}

impl MapEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_location_found(self, callback: impl Fn(LocationEvent) + 'static) -> Self {
        self.inner.borrow_mut().location_found = Some(Box::new(callback));
        self
    }

    pub fn with_location_error(self, callback: impl Fn(ErrorEvent) + 'static) -> Self {
        self.inner.borrow_mut().location_error = Some(Box::new(callback));
        self
    }

    pub fn with_popup_open(self, callback: impl Fn(PopupEvent) + 'static) -> Self {
        self.inner.borrow_mut().popup_open = Some(Box::new(callback));
        self
    }

    pub fn with_popup_close(self, callback: impl Fn(PopupEvent) + 'static) -> Self {
        self.inner.borrow_mut().popup_close = Some(Box::new(callback));
        self
    }

    pub fn with_load(self, callback: impl Fn(Event) + 'static) -> Self {
        self.inner.borrow_mut().load = Some(Box::new(callback));
        self
    }

    pub fn with_unload(self, callback: impl Fn(Event) + 'static) -> Self {
        self.inner.borrow_mut().unload = Some(Box::new(callback));
        self
    }

    pub fn with_resize(self, callback: impl Fn(Event) + 'static) -> Self {
        self.inner.borrow_mut().resize = Some(Box::new(callback));
        self
    }

    pub fn take_location_found(self) -> Option<Box<dyn Fn(LocationEvent)>> {
        self.inner.borrow_mut().location_found.take()
    }

    pub fn take_location_error(self) -> Option<Box<dyn Fn(ErrorEvent)>> {
        self.inner.borrow_mut().location_error.take()
    }

    pub fn take_popup_open(self) -> Option<Box<dyn Fn(PopupEvent)>> {
        self.inner.borrow_mut().popup_open.take()
    }

    pub fn take_popup_close(self) -> Option<Box<dyn Fn(PopupEvent)>> {
        self.inner.borrow_mut().popup_close.take()
    }

    pub fn take_load(self) -> Option<Box<dyn Fn(Event)>> {
        self.inner.borrow_mut().load.take()
    }

    pub fn take_unload(self) -> Option<Box<dyn Fn(Event)>> {
        self.inner.borrow_mut().unload.take()
    }

    pub fn take_resize(self) -> Option<Box<dyn Fn(Event)>> {
        self.inner.borrow_mut().resize.take()
    }
}
