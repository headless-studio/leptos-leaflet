mod bounds;
mod circle;
mod context;
mod control;
mod events;
mod image_overlay;
mod map_container;
mod marker;
mod pane;
mod pane_strategy;
mod path_options;
mod polygon;
mod polyline;
mod popup;
mod position;
mod quad_tile_layer;
mod tile_layer;
mod tile_layer_wms;
mod tooltip;
mod video_overlay;
mod zoom;

pub use bounds::Bounds;
pub use circle::Circle;
pub use context::*;
pub use control::Control;
pub use events::{
    DragEvents, LayerEvents, MapEvents, MouseEvents, MoveEvents, PopupEvents, TooltipEvents,
};
pub use image_overlay::ImageOverlay;
pub use leaflet::{CircleOptions, PathOptions, PolylineOptions};
pub use map_container::{LeafletMap, MapContainer};
pub use marker::Marker;
pub use pane::{
    provide_pane_context, provide_pane_context_with_renderer, use_pane_context, Pane, PaneContext,
    RendererType,
};
pub use pane_strategy::PaneStrategy;
pub use path_options::*;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use popup::Popup;
pub use position::*;
pub use quad_tile_layer::QuadTileLayer;
pub use tile_layer::TileLayer;
pub use tile_layer_wms::{TileLayerWms, TileLayerWmsEvents};
pub use tooltip::Tooltip;
pub use video_overlay::VideoOverlay;
pub use zoom::Zoom;

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
        $crate::paste! {
            create_effect(move |_| {
                use leaflet;
                let overlay_context =
                    leptos::use_context::<LeafletOverlayContainerContext>().expect("overlay context");
                if let (Some(layer), Some(setting)) = (overlay_context.container::<$class>(), &$value) {
                    let mut options = <$option_class>::new();
                    options.[<set_ $name>](&setting.get());
                    layer.set_style(&options);
                }
            });
        }
    };
}

#[macro_export]
macro_rules! setup_layer_option {
    ($name:ident, $options:ident) => {
        $crate::paste! {
            if let Some($name) = $name {
                $options.[<set_ $name>]($name.get_untracked());
            }
        }
    };
}

#[macro_export]
macro_rules! setup_layer_option_ref {
    ($name:ident, $options:ident) => {
        $crate::paste! {
            if let Some($name) = &$name {
                $options.[<set_ $name>](&$name.get_untracked());
            }
        }
    };
}

#[macro_export]
macro_rules! setup_layer_option_str {
    ($name:ident, $options:ident) => {
        $crate::paste! {
            if let Some($name) = &$name {
                $options.[<set_ $name>](&format!("{}", &$name.get_untracked()));
            }
        }
    };
}

#[macro_export]
macro_rules! setup_layer_leaflet_option {
    ($name:ident, $options:ident) => {
        $crate::paste! {
            if let Some($name) = $name.get_untracked() {
                $options.[<set_ $name>]($name);
            }
        }
    };
}

#[macro_export]
macro_rules! setup_layer_leaflet_option_ref {
    ($name:ident, $options:ident) => {
        $crate::paste! {
            if let Some($name) = $name.get_untracked() {
                $options.[<set_ $name>]($name.to_string());
            }
        }
    };
}

#[macro_export]
macro_rules! setup_layer_leaflet_string {
    ($name:ident, $options:ident) => {
        $crate::paste! {
            if !$name.get_untracked().is_empty() {
                $options.[<set_ $name>]($name.get_untracked().to_string());
            }
        }
    };
}

pub trait StringEmptyOption<T> {
    fn to_option(&self) -> Option<&T>;
    fn to_option_owned(self) -> Option<T>;
}

impl StringEmptyOption<String> for String {
    fn to_option(&self) -> Option<&String> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn to_option_owned(self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
