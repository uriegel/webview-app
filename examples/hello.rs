#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use webview_app::{application::Application, webview::WebView};

fn on_activate(app: &Application)->WebView {
    let can_close = true;

    let webview = WebView::builder(app)
        .title("Rust Web View 👍".to_string())
        .save_bounds()
        .debug_url("https://crates.io/crates/webview_app".to_string())
        .url("https://crates.io/crates".to_string())
        .devtools(true)
        .default_contextmenu_disabled()
        .build();

    webview.can_close(move ||can_close);
    webview
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}
