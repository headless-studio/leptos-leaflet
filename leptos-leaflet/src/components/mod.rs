mod circle;
mod context;
mod events;
mod image_overlay;
mod map_container;
mod marker;
mod path_options;
mod polygon;
mod polyline;
mod popup;
mod position;
mod tile_layer;
mod tooltip;
mod video_overlay;

pub use circle::Circle;
pub use context::*;
pub use events::{
    DragEvents, LayerEvents, MapEvents, MouseEvents, MoveEvents, PopupEvents, TooltipEvents,
};
pub use leaflet::{CircleOptions, PathOptions, PolylineOptions};
pub use map_container::{LeafletMap, MapContainer};
pub use marker::Marker;
pub use path_options::*;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use popup::Popup;
pub use position::Position;
pub use tile_layer::TileLayer;
pub use tooltip::Tooltip;

#[macro_export]
macro_rules! effect_update_on_change {
    ($class:ty, $option_class:ty, $name:ident) => {
        create_effect(move |_| {
            use leaflet;
            let overlay_context =
                leptos::use_context::<LeafletOverlayContainerContext>().expect("overlay context");
            if let (Some(layer), Some(setting)) = (overlay_context.container::<$class>(), $name) {
                let mut options = <$option_class>::new();
                options.$name(setting.get());
                layer.setStyle(&options);
            }
        });
    };
}

#[macro_export]
macro_rules! effect_update_on_change_ref {
    ($class:ty, $option_class:ty, $name:ident, $value:expr) => {
        create_effect(move |_| {
            use leaflet;
            let overlay_context =
                leptos::use_context::<LeafletOverlayContainerContext>().expect("overlay context");
            if let (Some(layer), Some(setting)) = (overlay_context.container::<$class>(), &$value) {
                let mut options = <$option_class>::new();
                options.$name(&setting.get());
                layer.setStyle(&options);
            }
        });
    };
}

#[macro_export]
macro_rules! setup_layer_option {
    ($name:ident, $options:ident) => {
        if let Some($name) = $name {
            $options.$name($name.get_untracked());
        }
    };
}

#[macro_export]
macro_rules! setup_layer_option_ref {
    ($name:ident, $options:ident) => {
        if let Some($name) = &$name {
            $options.$name(&$name.get_untracked());
        }
    };
}

#[macro_export]
macro_rules! setup_layer_option_str {
    ($name:ident, $options:ident) => {
        if let Some($name) = &$name {
            $options.$name(&format!("{}", &$name.get_untracked()));
        }
    };
}

#[macro_export]
macro_rules! setup_layer_leaflet_option {
    ($name:ident, $options:ident) => {
        if let Some($name) = $name.get_untracked() {
            $options.$name($name);
        }
    };
}

#[macro_export]
macro_rules! setup_layer_leaflet_option_ref {
    ($name:ident, $options:ident) => {
        if let Some($name) = $name.get_untracked() {
            $options.$name($name.as_ref());
        }
    };
}
