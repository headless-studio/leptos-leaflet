use crate::leaflet_event;
use leaflet::MouseEvent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct MouseEvents {
    inner: Rc<RefCell<InnerMouseEvents>>,
}

#[derive(Default)]
struct InnerMouseEvents {
    on_click: Option<Box<dyn Fn(MouseEvent)>>,
    on_double_click: Option<Box<dyn Fn(MouseEvent)>>,
    on_mouse_down: Option<Box<dyn Fn(MouseEvent)>>,
    on_mouse_up: Option<Box<dyn Fn(MouseEvent)>>,
    on_mouse_over: Option<Box<dyn Fn(MouseEvent)>>,
    on_mouse_out: Option<Box<dyn Fn(MouseEvent)>>,
    on_context_menu: Option<Box<dyn Fn(MouseEvent)>>,
}

impl MouseEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, evented: &impl leaflet::MouseEvents) {
        if let Some(on_click) = self.inner.borrow_mut().on_click.take() {
            evented.on_click(on_click);
        }
        if let Some(on_double_click) = self.inner.borrow_mut().on_double_click.take() {
            evented.on_double_click(on_double_click);
        }
        if let Some(on_mouse_down) = self.inner.borrow_mut().on_mouse_down.take() {
            evented.on_mouse_down(on_mouse_down);
        }
        if let Some(on_mouse_up) = self.inner.borrow_mut().on_mouse_up.take() {
            evented.on_mouse_up(on_mouse_up);
        }
        if let Some(on_mouse_over) = self.inner.borrow_mut().on_mouse_over.take() {
            evented.on_mouse_over(on_mouse_over);
        }
        if let Some(on_mouse_out) = self.inner.borrow_mut().on_mouse_out.take() {
            evented.on_mouse_out(on_mouse_out);
        }
        if let Some(on_context_menu) = self.inner.borrow_mut().on_context_menu.take() {
            evented.on_context_menu(on_context_menu);
        }
    }
}

leaflet_event!(MouseEvents, on_click, MouseEvent);
leaflet_event!(MouseEvents, on_double_click, MouseEvent);
leaflet_event!(MouseEvents, on_mouse_down, MouseEvent);
leaflet_event!(MouseEvents, on_mouse_up, MouseEvent);
leaflet_event!(MouseEvents, on_mouse_over, MouseEvent);
leaflet_event!(MouseEvents, on_mouse_out, MouseEvent);
leaflet_event!(MouseEvents, on_context_menu, MouseEvent);
