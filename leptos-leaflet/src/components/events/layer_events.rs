use crate::leaflet_events;
use leaflet::Event;

leaflet_events!(
    (LayerEvents, impl leaflet::LayerEvents),
    (add, add, Event),
    (remove, remove, Event)
);
