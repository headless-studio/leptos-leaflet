use crate::leaflet_events;
use leaflet::MouseEvent;

leaflet_events!(
    (MouseEvents, impl leaflet::MouseEvents),
    (click, click, MouseEvent),
    (double_click, double_click, MouseEvent),
    (mouse_down, mouse_down, MouseEvent),
    (mouse_up, mouse_up, MouseEvent),
    (mouse_over, mouse_over, MouseEvent),
    (mouse_out, mouse_out, MouseEvent),
    (context_menu, context_menu, MouseEvent)
);
