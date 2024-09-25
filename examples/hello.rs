#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use webview_app::webview::WebView;

fn main() {
    let webview = 
        WebView::builder()
            .appid("de.uriegel.hello".to_string())
            .title("Rust Web View 👍".to_string())
            .initial_bounds(400, 600)
            .url("https://crates.io/crates/webview_app".to_string())
            .build();
    webview.run();
}