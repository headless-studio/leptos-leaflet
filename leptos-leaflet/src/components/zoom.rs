use leaflet::{Zoom, ZoomOptions};
use leptos::prelude::*;

use crate::prelude::LeafletMapContext;

/// Zoom controls
#[component]
#[allow(unused_braces)]
pub fn Zoom(
    /// Position where the zoom control is shown.
    #[prop(into)]
    position: Signal<String>,
    /// Text for "zoom in".
    #[prop(optional)]
    zoom_in_text: Option<String>,
    /// Text for "zoom out".
    #[prop(optional)]
    zoom_out_text: Option<String>,
    /// Title for "zoom in".
    #[prop(optional)]
    zoom_in_title: Option<String>,
    /// Title for "zoom out".
    #[prop(optional)]
    zoom_out_title: Option<String>,
) -> impl IntoView {
    let control = RwSignal::new_local(None::<Zoom>);

    let mut options = ZoomOptions::new();
    options.set_position(&position.get_untracked());

    if let Some(in_text) = zoom_in_text {
        options.set_zoom_in_text(&in_text);
    }
    if let Some(out_text) = zoom_out_text {
        options.set_zoom_out_text(&out_text);
    }
    if let Some(in_title) = zoom_in_title {
        options.set_zoom_in_title(&in_title);
    }
    if let Some(out_title) = zoom_out_title {
        options.set_zoom_out_title(&out_title);
    }

    Effect::new(move |_| {
        let Some(map) = use_context::<LeafletMapContext>()
            .expect("Leaflet context not available. Could not initialize Zoom component.")
            .map()
        else {
            return;
        };

        let c = Zoom::new(&options);
        c.add_to(&map);
        control.set(Some(c));
    });

    on_cleanup(move || {
        control.with_untracked(|contr| {
            if let Some(c) = contr {
                c.remove();
            }
        });
    });

    let update_position = move || {
        let position = position.get();
        let Some(c) = control.get() else {
            return;
        };
        c.set_position(&position);
    };

    view! {
        { update_position }
    }
}
