#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use webview_app::{application::Application, webview::WebView};

fn on_activate(app: &Application) {
    let can_close = true;

    WebView::builder(app)
        .title("Rust Web View 👍".to_string())
        .save_bounds()
        .debug_url("https://crates.io/crates/webview_app".to_string())
        .url("https://crates.io/crates".to_string())
        .devtools(true)
        .default_contextmenu_disabled()
        .can_close(move||{
            can_close
        })
        .build();
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}
