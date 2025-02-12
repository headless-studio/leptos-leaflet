use leptos::prelude::*;

mod app;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {<App/>}
    });
}
