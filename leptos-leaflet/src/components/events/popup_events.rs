use crate::leaflet_events;
use leaflet::PopupEvent;

leaflet_events!(
    (PopupEvents, impl leaflet::PopupEvents),
    (popup_open, popup_open, PopupEvent),
    (popup_close, popup_close, PopupEvent)
);
