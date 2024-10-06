use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
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
            //         //    MainContext::default().spawn_local(async move {
            webview.webview.webview.webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
    });
} 

// fn request_blocking<R: 'static, F: std::future::Future<Output = R> + 'static>(
//     on_request: F) {
//         spawn_future_local(async move {
//             let response = gio::spawn_blocking( move|| {
//                 on_request
//     });
// } 

