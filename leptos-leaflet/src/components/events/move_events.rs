use crate::leaflet_events;
use leaflet::Event;

leaflet_events!(
    (MoveEvents, impl leaflet::MoveEvents),
    (move, move, Event)
);
