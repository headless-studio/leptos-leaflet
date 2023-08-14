use crate::leaflet_event;
use leaflet::{DragEndEvent, Event};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct DragEvents {
    inner: Rc<RefCell<InnerDragEvents>>,
}

#[derive(Default)]
struct InnerDragEvents {
    on_drag_start: Option<Box<dyn Fn(Event)>>,
    on_move_start: Option<Box<dyn Fn(Event)>>,
    on_drag: Option<Box<dyn Fn(Event)>>,
    on_drag_end: Option<Box<dyn Fn(DragEndEvent)>>,
    on_move_end: Option<Box<dyn Fn(Event)>>,
}

impl DragEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, evented: &impl leaflet::DragEvents) {
        if let Some(on_drag_start) = self.inner.borrow_mut().on_drag_start.take() {
            evented.on_drag_start(on_drag_start);
        }
        if let Some(on_move_start) = self.inner.borrow_mut().on_move_start.take() {
            evented.on_move_start(on_move_start);
        }
        if let Some(on_drag) = self.inner.borrow_mut().on_drag.take() {
            evented.on_drag(on_drag);
        }
        if let Some(on_drag_end) = self.inner.borrow_mut().on_drag_end.take() {
            evented.on_drag_end(on_drag_end);
        }
        if let Some(on_move_end) = self.inner.borrow_mut().on_move_end.take() {
            evented.on_move_end(on_move_end);
        }
    }
}

leaflet_event!(DragEvents, on_drag_start, Event);
leaflet_event!(DragEvents, on_move_start, Event);
leaflet_event!(DragEvents, on_drag, Event);
leaflet_event!(DragEvents, on_drag_end, DragEndEvent);
leaflet_event!(DragEvents, on_move_end, Event);
