use crate::LeafletMapContext;
use leaflet::Control;
use leaflet::ControlOptions;
use leptos::*;

/// Creates a new control.
#[component]
#[allow(unused_braces)]
pub fn Control(
    /// Wether the container should get the class `leaflet-bar`.
    #[prop(optional, default = false)]
    leaflet_bar: bool,
    /// Position of the control.
    #[prop(optional, into, default = MaybeSignal::Static("topleft".to_string()))]
    position: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let control_view = view! {
        <div
            class:leaflet-bar=leaflet_bar>
        { children() }
        </div>
    };

    let control = store_value(None::<Control>);

    let options = ControlOptions::new();
    options.set_position(position.get_untracked());

    create_effect(move |_| {
        let Some(map) = use_context::<LeafletMapContext>()
            .expect("Leaflet context not available. Could not initialize Control component.")
            .map()
        else {
            return;
        };

        let c = Control::new(&options);

        let control_view = control_view.clone();
        c.on_add(move |_| (**control_view).clone());

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
