#[cfg(target_os = "linux")]
use gtk::gio;

use serde::{Deserialize, Serialize};

/// To get the deserialized payload from a JSON request
pub fn get_input<'a, T>(data: &'a str)->T where T: Deserialize<'a> {
    serde_json::from_str(data).unwrap()
}

// To serialize payload result for a JSON request
pub fn get_output<T>(result: &T)->String where T: Serialize {
    serde_json::to_string(result).unwrap()
}

/// This struct is used as handle for "request_async" and "request_blocking"
#[cfg(target_os = "linux")]
pub struct Request {
    pub(crate) webview: webkit6::WebView
}

/// This struct is used as handle for "request_async" and "request_blocking"
#[cfg(target_os = "windows")]
pub struct Request {
    pub(crate) hwnd: isize
}

/// Handling an incoming request from javascript.
/// 
/// The callback function has to be non blocking and async
/// 
/// If an exception occurs, the error text is written to the error console
#[cfg(target_os = "linux")]
pub fn request_async<F: std::future::Future<Output = String> + 'static>(
    request: &Request, id: String, on_request: F) {
        use gtk::glib::spawn_future_local;
        use webkit6::prelude::*;

        let webview = request.webview.clone();
        spawn_future_local(async move {
            let response = on_request.await;
            let back: String = get_back(id, response);
            let res = webview
                .evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None)
                .await;
            let _r = res.inspect_err(|err|eprintln!("Error executing script: {:?}", err));
                
    });
} 

/// Handling an incoming request from javascript in a blocking manner in a threadpool thread.
/// 
/// The calling function does not block, the callback function runs in a threadpool thread and can be blocking.
/// 
/// If an exception occurs, the error text is written to the error console
#[cfg(target_os = "linux")]
pub fn request_blocking<F: FnOnce() -> String + Send + 'static>(
    request: &Request, id: String, on_request: F) {
        use gtk::glib::spawn_future_local;
        use webkit6::prelude::*;

        let webview = request.webview.clone();
        spawn_future_local(async move {
            let response = gio::spawn_blocking(move|| {
                let res = on_request();
                res
            }).await.expect("Task needs to finish successfully.");

            let back: String = get_back(id, response);
            let res = webview
                .evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None)
                .await;
            let _r = res.inspect_err(|err|eprintln!("Error executing script blocking: {:?}", err));
    });
} 

/// Handling an incoming request from javascript in a blocking manner in a thread.
/// 
/// The calling function does not block, the callback function runs in a thread and can be blocking.
/// 
/// If an exception occurs, the error text is written to the error console
#[cfg(target_os = "windows")]
pub fn request_blocking<F: FnOnce() -> String + Send + 'static>(
    request: &Request, id: String, on_request: F) {
        use std::{ffi::c_void, thread};
        use webview2_com::CoTaskMemPWSTR;
        use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
        use windows::Win32::UI::WindowsAndMessaging::PostMessageW;

        use crate::windows::appmessage::APP_SENDRESPONSE;

        let hwnd = request.hwnd;

        thread::spawn(move|| {
            let response = on_request();
            let back: String = format!("result,{},{}", id, response);
            let mut back = CoTaskMemPWSTR::from(back.as_str());
            let wparam: WPARAM = WPARAM(back.take().as_ptr() as usize); 
            let lparam: LPARAM = LPARAM(0);   
            let hwnd = hwnd as *mut c_void;
            let hwnd = HWND(hwnd);
            let res = unsafe { PostMessageW(hwnd, APP_SENDRESPONSE, wparam, lparam) };
            let _r = res.inspect_err(|err|eprintln!("Error executing script blocking: {:?}", err));
        });
} 

#[cfg(target_os = "linux")]
fn get_back(id: String, response: String)->String {
    let back: String = format!("result,{},{}", id, response);
    back.replace("'", "u0027")
}
