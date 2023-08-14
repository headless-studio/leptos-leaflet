use crate::leaflet_event;
use leaflet::Event;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct MoveEvents {
    inner: Rc<RefCell<InnerMoveEvents>>,
}

#[derive(Default)]
struct InnerMoveEvents {
    on_move: Option<Box<dyn Fn(Event)>>,
}

impl MoveEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, evented: &impl leaflet::MoveEvents) {
        if let Some(on_move) = self.inner.borrow_mut().on_move.take() {
            evented.on_move(on_move);
        }
    }
}

leaflet_event!(MoveEvents, on_move, Event);
