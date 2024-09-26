#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use webview_app::webview::WebView;

fn main() {
    let can_close = true;

    let webview = 
        WebView::builder()
            .appid("de.uriegel.hello".to_string())
            .title("Rust Web View 👍".to_string())
            .save_bounds()
            .url("https://crates.io/crates/webview_app".to_string())
            .devtools()
            .default_contextmenu_disabled()
            .can_close(move||{
                can_close
            })
            .build();
    webview.run();
}