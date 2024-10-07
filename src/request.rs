#[cfg(target_os = "linux")]
use gtk::gio;

use serde::{Deserialize, Serialize};

use crate::webview::WebView;

pub fn get_input<'a, T>(data: &'a str)->T where T: Deserialize<'a> {
    serde_json::from_str(data).unwrap()
}

pub fn get_output<T>(result: &T)->String where T: Serialize {
    serde_json::to_string(result).unwrap()
}

#[cfg(target_os = "linux")]
pub fn request_async<F: std::future::Future<Output = String> + 'static>(
    webview: WebView, id: String, on_request: F) {
        use gtk::glib::spawn_future_local;
        use webkit6::prelude::*;
        spawn_future_local(async move {
            let response = on_request.await;
            let back: String = format!("result,{},{}", id, response);
            webview.webview.webview.webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
    });
} 

#[cfg(target_os = "linux")]
pub fn request_blocking<F: FnOnce() -> String + Send + 'static>(
    webview: WebView, id: String, on_request: F) {
        use gtk::glib::spawn_future_local;
        use webkit6::prelude::*;

        spawn_future_local(async move {
            let response = gio::spawn_blocking(move|| {
                let res = on_request();
                res
            }).await.expect("Task needs to finish successfully.");

            let back: String = format!("result,{},{}", id, response);
            webview.webview.webview.webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
    });
} 


#[cfg(target_os = "windows")]
pub fn request_blocking<F: FnOnce() -> String + Send + 'static>(
    _: WebView, id: String, on_request: F) {
        use std::thread;
        use crate::windows::raw_funcs::load_raw_funcs;
        thread::spawn(move|| {
            let response = on_request();
            let back: String = format!("result,{},{}", id, response);
            (load_raw_funcs("").send_text)(back.as_ptr());
        });
} 


