use crate::leaflet_event;
use leaflet::TooltipEvent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct TooltipEvents {
    inner: Rc<RefCell<InnerTooltipEvents>>,
}

#[derive(Default)]
struct InnerTooltipEvents {
    on_tooltip_open: Option<Box<dyn Fn(TooltipEvent)>>,
    on_tooltip_close: Option<Box<dyn Fn(TooltipEvent)>>,
}

impl TooltipEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, evented: &impl leaflet::TooltipEvents) {
        if let Some(on_tooltip_open) = self.inner.borrow_mut().on_tooltip_open.take() {
            evented.on_tooltip_open(on_tooltip_open);
        }
        if let Some(on_tooltip_close) = self.inner.borrow_mut().on_tooltip_close.take() {
            evented.on_tooltip_close(on_tooltip_close);
        }
    }
}

leaflet_event!(TooltipEvents, on_tooltip_open, TooltipEvent);
leaflet_event!(TooltipEvents, on_tooltip_close, TooltipEvent);
