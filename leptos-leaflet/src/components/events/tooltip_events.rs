use crate::leaflet_events;
use leaflet::TooltipEvent;

leaflet_events!(
    (TooltipEvents, impl leaflet::TooltipEvents),
    (tooltip_open, tooltip_open, TooltipEvent),
    (tooltip_close, tooltip_close, TooltipEvent)
);
