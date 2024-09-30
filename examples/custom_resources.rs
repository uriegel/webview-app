#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use include_dir::include_dir;
use webview_app::webview::WebView;

fn main() {
    let webview = 
        WebView::builder()
            .appid("de.uriegel.hello".to_string())
            .title("Website form custom resources 👍".to_string())
            .save_bounds()
            .devtools(true)
            .webroot(include_dir!("webroots/custom_resources"))
            .default_contextmenu_disabled()
            .build();
    webview.run();
}