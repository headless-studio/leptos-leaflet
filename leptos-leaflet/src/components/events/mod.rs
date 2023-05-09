mod drag_events;
mod layer_events;
mod map_events;
mod mouse_events;
mod popup_events;
mod tooltip_events;

pub use drag_events::DragEvents;
pub use layer_events::LayerEvents;
pub use map_events::MapEvents;
pub use mouse_events::MouseEvents;
pub use popup_events::PopupEvents;
pub use tooltip_events::TooltipEvents;

#[macro_export]
macro_rules! leaflet_event {
    ($s:ident, $e:ident, $t:ty) => {
        impl $s {
            /// Set ups event $n
            pub fn $e(self, callback: impl Fn($t) + 'static) -> Self {
                self.inner.borrow_mut().$e = Some(Box::new(callback));
                self
            }
        }
    };
}
