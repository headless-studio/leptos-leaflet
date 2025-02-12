use leptos::prelude::*;

mod app;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();

    let config = wasm_logger::Config::default();
    wasm_logger::init(config);
    log::info!("Logging initialized.");

    mount_to_body(|| {
        view! {<App/>}
    });
}
