use dioxus_live::app;

fn main() {
    start();
}

#[cfg(feature = "web")]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

#[cfg(feature = "desktop")]
pub fn start() {
    tracing_subscriber::fmt::init();
    dioxus::desktop::launch(app);
}
