use crate::components::context::extend_context_with_overlay;
use crate::components::position::Position;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::{
    DragEvents, LayerEvents, LeafletMapContext, MouseEvents, MoveEvents, PopupEvents, TooltipEvents,
};
use crate::core::{JsMaybeSignal, JsStoredValue};
use crate::{setup_layer_leaflet_option, setup_layer_leaflet_string};

/// A marker component.
#[component(transparent)]
pub fn Marker(
    /// Position for the Marker
    #[prop(into)]
    position: JsMaybeSignal<Position>,
    #[prop(into, optional)] draggable: MaybeSignal<bool>,
    #[prop(into, optional)] keyboard: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] title: MaybeSignal<String>,
    #[prop(into, optional)] alt: MaybeSignal<String>,
    #[prop(into, optional)] interactive: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] z_index_offset: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] opacity: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] rise_on_hover: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] rise_offset: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] pane: MaybeSignal<String>,
    #[prop(into, optional)] shadow_pane: MaybeSignal<String>,
    #[prop(into, optional)] bubbling_mouse_events: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] auto_pan: MaybeSignal<Option<bool>>,
    #[prop(into, optional)] auto_pan_padding: MaybeSignal<Option<(f64, f64)>>,
    #[prop(into, optional)] auto_pan_speed: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] icon_class: MaybeSignal<Option<String>>,
    #[prop(into, optional)] icon_url: MaybeSignal<Option<String>>,
    #[prop(into, optional)] icon_size: MaybeSignal<Option<(f64, f64)>>,
    #[prop(into, optional)] icon_anchor: MaybeSignal<Option<(f64, f64)>>,
    #[prop(into, optional)] attribution: MaybeSignal<String>,
    #[prop(into, optional)] rotation: MaybeSignal<Option<f64>>,
    #[prop(into, optional)] move_events: MoveEvents,
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] drag_events: DragEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let position_tracking = position;
    let icon_class_tracking = icon_class.clone();
    let icon_url_tracking = icon_url.clone();
    let icon_size_tracking = icon_size;
    let icon_anchor_tracking = icon_anchor;
    let map_context = use_context::<LeafletMapContext>().expect("Map context not found");

    let overlay_context = extend_context_with_overlay();
    let overlay = JsStoredValue::new_local(None::<leaflet::Marker>);

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            let options = leaflet::MarkerOptions::new();
            let drag = draggable.get_untracked();
            if drag {
                options.set_draggable(drag);
            }
            setup_layer_leaflet_option!(keyboard, options);
            setup_layer_leaflet_string!(title, options);
            setup_layer_leaflet_string!(alt, options);
            setup_layer_leaflet_option!(interactive, options);
            setup_layer_leaflet_option!(z_index_offset, options);
            setup_layer_leaflet_option!(opacity, options);
            setup_layer_leaflet_option!(rise_on_hover, options);
            setup_layer_leaflet_option!(rise_offset, options);
            setup_layer_leaflet_string!(pane, options);
            setup_layer_leaflet_string!(shadow_pane, options);
            setup_layer_leaflet_option!(bubbling_mouse_events, options);
            setup_layer_leaflet_option!(auto_pan, options);
            setup_layer_leaflet_option!(auto_pan_speed, options);
            setup_layer_leaflet_string!(attribution, options);

            if let Some((x, y)) = auto_pan_padding.get_untracked() {
                options.set_auto_pan_padding(leaflet::Point::new(x, y));
            }
            if let Some(icon_url) = icon_url.get_untracked() {
                let icon_options = leaflet::IconOptions::new();
                icon_options.set_icon_url(icon_url);
                if let Some((x, y)) = icon_size.get_untracked() {
                    icon_options.set_icon_size(leaflet::Point::new(x, y));
                }
                if let Some((x, y)) = icon_anchor.get_untracked() {
                    icon_options.set_icon_anchor(leaflet::Point::new(x, y));
                }
                let icon = leaflet::Icon::new(&icon_options);
                options.set_icon(icon);
            } else if let Some(icon_class) = icon_class.get_untracked() {
                let icon_options = leaflet::DivIconOptions::new();
                icon_options.set_class_name(icon_class);
                if let Some((x, y)) = icon_size.get_untracked() {
                    icon_options.set_icon_size(leaflet::Point::new(x, y));
                }
                if let Some((x, y)) = icon_anchor.get_untracked() {
                    icon_options.set_icon_anchor(leaflet::Point::new(x, y));
                }
                let icon = leaflet::DivIcon::new(&icon_options);
                options.set_icon(icon.into());
            }

            let marker =
                leaflet::Marker::new_with_options(&position.get_untracked().as_lat_lng(), &options);

            mouse_events.setup(&marker);
            move_events.setup(&marker);
            drag_events.setup(&marker);
            popup_events.setup(&marker);
            tooltip_events.setup(&marker);
            layer_events.setup(&marker);

            marker.add_to(&map);
            overlay_context.set_container(&marker);
            overlay.set_value(Some(marker));
        };
    });

    let position_stop = Effect::watch(
        move || position_tracking.get(),
        move |position_tracking, _, _| {
            if let Some(marker) = overlay.get_value().as_ref() {
                marker.set_lat_lng(&position_tracking.as_lat_lng());
            }
        },
        false,
    );

    let icon_stop = Effect::watch(
        move || {
            (
                icon_url_tracking.get(),
                icon_class_tracking.get(),
                icon_size_tracking.get(),
                icon_anchor_tracking.get(),
            )
        },
        move |(maybe_icon_url, maybe_icon_class, maybe_icon_size, maybe_icon_anchor), _, _| {
            if let Some(marker) = overlay.get_value() {
                if let Some(icon_url) = maybe_icon_url {
                    let icon_options = leaflet::IconOptions::new();
                    icon_options.set_icon_url(icon_url.clone());
                    if let Some((x, y)) = maybe_icon_size {
                        icon_options.set_icon_size(leaflet::Point::new(*x, *y));
                    }
                    if let Some((x, y)) = maybe_icon_anchor {
                        icon_options.set_icon_anchor(leaflet::Point::new(*x, *y));
                    }
                    let icon = leaflet::Icon::new(&icon_options);
                    marker.set_icon(&icon);
                } else if let Some(icon_class) = maybe_icon_class {
                    let icon_options = leaflet::DivIconOptions::new();
                    icon_options.set_class_name(icon_class.clone());
                    if let Some((x, y)) = maybe_icon_size {
                        icon_options.set_icon_size(leaflet::Point::new(*x, *y));
                    }
                    if let Some((x, y)) = maybe_icon_anchor {
                        icon_options.set_icon_anchor(leaflet::Point::new(*x, *y));
                    }
                    let icon = leaflet::DivIcon::new(&icon_options);
                    marker.set_icon(&icon.into());
                }
            }
        },
        false,
    );

    let opacity_stop = Effect::watch(
        move || opacity.get(),
        move |opacity, _, _| {
            overlay.get_value().as_ref().and_then(|marker| {
                opacity.map(|opacity| {
                    marker.set_opacity(opacity);
                })
            });
        },
        false,
    );

    let drag_stop = Effect::watch(
        move || draggable.get(),
        move |&draggable, _, _| {
            if let Some(marker) = overlay.get_value().as_ref() {
                match draggable {
                    true => marker.dragging().enable(),
                    false => marker.dragging().disable(),
                };
            }
        },
        false,
    );

    let rotation_stop = Effect::watch(
        move || rotation.get(),
        move |&rotation, prev_rotation, _| {
            if let (Some(marker), Some(rotation)) = (overlay.get_value().as_ref(), rotation) {
                if Some(rotation.trunc()) == prev_rotation.copied().flatten().map(|r| r.trunc()) {
                    return;
                }
                if let Ok(internal_icon) = js_sys::Reflect::get(marker, &"_icon".into()) {
                    let internal_icon = internal_icon.unchecked_ref::<web_sys::HtmlElement>();
                    _ = internal_icon
                        .style()
                        .set_property("--gps_rotation", &format!("{}deg", rotation));

                    let transform = internal_icon
                        .style()
                        .get_property_value("transform")
                        .unwrap_or_default();

                    let transform = match transform.contains("rotate(") {
                        true => transform
                            .split_whitespace()
                            .map(|part| match part.starts_with("rotate(") {
                                true => format!("rotate({}deg)", rotation),
                                false => part.to_string(),
                            })
                            .collect::<Vec<String>>()
                            .join(" "),
                        false => format!("{} rotate({}deg)", transform, rotation),
                    };

                    let _ = internal_icon.style().set_property("transform", &transform);
                    let _ = internal_icon
                        .style()
                        .set_property("transform-origin", "center");
                }
            }
        },
        false,
    );

    on_cleanup(move || {
        position_stop.stop();
        icon_stop.stop();
        opacity_stop.stop();
        drag_stop.stop();
        rotation_stop.stop();
        if let Some(overlay) = overlay.get_value() {
            overlay.remove();
        }
    });

    children.map(|child| child())
}
