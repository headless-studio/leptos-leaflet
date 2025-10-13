use crate::leaflet_events;
use leaflet::{ErrorEvent, Event, LocationEvent, Map, MouseEvent};

leaflet_events!(
    (MapEvents, Map),
    (location_found, location_found, LocationEvent),
    (location_error, location_error, ErrorEvent),
    (load, load, Event),
    (unload, unload, Event),
    (resize, resize, Event),
    (zoom, zoom, Event),
    (zoom_start, zoom_start, Event),
    (zoom_end, zoom_end, Event),
    (move, move, Event),
    (move_start, move_start, Event),
    (move_end, move_end, Event),
    (mouse_click, mouse_click, MouseEvent),
    (mouse_double_click, mouse_double_click, MouseEvent),
    (mouse_context_menu, mouse_context_menu, MouseEvent),
    (mouse_move, mouse_move, MouseEvent),
    (mouse_over, mouse_over, MouseEvent),
    (mouse_out, mouse_out, MouseEvent),
    (mouse_down, mouse_down, MouseEvent),
    (mouse_up, mouse_up, MouseEvent)
);
