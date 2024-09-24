#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use webview_app::webview::WebView;

fn main() {
    let webview = 
        WebView::builder()
            .title(String::from("Rust Web View 👍"))
            .build();
    webview.run();
}