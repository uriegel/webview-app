#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use webview_app::webview::WebView;

fn main() {
    let webview = 
        WebView::builder()
            // TODO builder pattern enhancement
            .appid(String::from("de.uriegel.hello"))
            .title(String::from("Rust Web View 👍"))
            .url(String::from("https://crates.io/crates/webview_app"))
            .build();
    webview.run();
}