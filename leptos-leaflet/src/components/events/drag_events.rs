use crate::leaflet_events;
use leaflet::{DragEndEvent, Event};
use leptos::prelude::LocalStorage;

leaflet_events!(
    (DragEvents, impl leaflet::DragEvents),
    (drag_start, drag_start, Event),
    (move_start, move_start, Event),
    (drag, drag, Event),
    (drag_end, drag_end, DragEndEvent),
    (move_end, move_end, Event)
);
