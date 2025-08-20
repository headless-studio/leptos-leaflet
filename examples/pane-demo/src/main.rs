use leptos::prelude::*;
use tracing::trace;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::MakeWebConsoleWriter;

mod app;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .without_time() // std::time is not available in browsers
        .with_writer(MakeWebConsoleWriter::new()); // write events to the console
    tracing_subscriber::registry().with(fmt_layer).init();
    trace!("Logging initialized.");

    mount_to_body(|| {
        view! {<App/>}
    });
}
