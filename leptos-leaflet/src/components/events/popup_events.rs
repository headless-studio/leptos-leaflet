use crate::leaflet_event;
use leaflet::PopupEvent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct PopupEvents {
    inner: Rc<RefCell<InnerPopupEvents>>,
}

#[derive(Default)]
struct InnerPopupEvents {
    on_popup_open: Option<Box<dyn Fn(PopupEvent)>>,
    on_popup_close: Option<Box<dyn Fn(PopupEvent)>>,
}

impl PopupEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, evented: &impl leaflet::PopupEvents) {
        if let Some(on_popup_open) = self.inner.borrow_mut().on_popup_open.take() {
            evented.on_popup_open(on_popup_open);
        }
        if let Some(on_popup_close) = self.inner.borrow_mut().on_popup_close.take() {
            evented.on_popup_close(on_popup_close);
        }
    }
}

leaflet_event!(PopupEvents, on_popup_open, PopupEvent);
leaflet_event!(PopupEvents, on_popup_close, PopupEvent);
