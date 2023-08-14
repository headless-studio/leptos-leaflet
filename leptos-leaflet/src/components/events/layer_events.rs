use crate::leaflet_event;
use leaflet::Event;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct LayerEvents {
    inner: Rc<RefCell<InnerLayerEvents>>,
}

#[derive(Default)]
struct InnerLayerEvents {
    on_add: Option<Box<dyn Fn(Event)>>,
    on_remove: Option<Box<dyn Fn(Event)>>,
}

impl LayerEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, evented: &impl leaflet::LayerEvents) {
        if let Some(on_add) = self.inner.borrow_mut().on_add.take() {
            evented.on_add(on_add);
        }
        if let Some(on_remove) = self.inner.borrow_mut().on_remove.take() {
            evented.on_remove(on_remove);
        }
    }
}

leaflet_event!(LayerEvents, on_add, Event);
leaflet_event!(LayerEvents, on_remove, Event);
