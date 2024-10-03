#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use include_dir::include_dir;
use webview_app::{application::Application, webview::WebView};

fn on_activate(app: &Application) {
    WebView::builder(app)
        .title("Website form custom resources 👍".to_string())
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/custom_resources"))
        .default_contextmenu_disabled()
        .build();
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}


