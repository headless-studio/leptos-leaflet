use leaflet::Control;
use leaflet::ControlOptions;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use crate::prelude::LeafletMapContext;

/// Creates a new control.
#[component]
pub fn Control(
    /// Wether the container should get the class `leaflet-bar`.
    #[prop(optional, default = false)]
    leaflet_bar: bool,
    /// Position of the control.
    #[prop(optional, into, default = Signal::derive(|| "topleft".to_string()))]
    position: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let control = StoredValue::new_local(None::<Control>);

    let options = ControlOptions::new();
    options.set_position(position.get_untracked());

    Effect::new(move |_| {
        let Some(map) = use_context::<LeafletMapContext>()
            .expect("Leaflet context not available. Could not initialize Control component.")
            .map()
        else {
            return;
        };

        let c = Control::new(&options);

        // Renders the children of the control.
        let control_view = view! {
            { children() }
        }
        .to_html();

        c.on_add(move |_| {
            let control_html: HtmlDivElement = document()
                .create_element("div")
                .expect("Could not create element.")
                .unchecked_into();
            if leaflet_bar {
                control_html.set_class_name("leaflet-bar");
            }
            control_html.set_inner_html(&control_view);

            control_html.unchecked_into()
        });

        c.add_to(&map);
        control.set_value(Some(c));
    });

    on_cleanup(move || {
        control.with_value(|contr| {
            if let Some(c) = contr {
                c.remove();
            }
        });
        control.set_value(None);
    });

    let update_position = move || {
        let position = position.get();
        let Some(c) = control.get_value() else {
            return;
        };
        c.set_position(&position);
    };

    view! {
        { update_position }
    }
}
