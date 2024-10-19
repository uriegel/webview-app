#[cfg(target_os = "linux")]
use gtk::gio;

use serde::{Deserialize, Serialize};

pub fn get_input<'a, T>(data: &'a str)->T where T: Deserialize<'a> {
    serde_json::from_str(data).unwrap()
}

pub fn get_output<T>(result: &T)->String where T: Serialize {
    serde_json::to_string(result).unwrap()
}

#[cfg(target_os = "linux")]
pub struct Request {
    pub(crate) webview: webkit6::WebView
}

#[cfg(target_os = "windows")]
pub struct Request {
    pub(crate) hwnd: isize
}

#[cfg(target_os = "linux")]
pub fn request_async<F: std::future::Future<Output = String> + 'static>(
    request: &Request, id: String, on_request: F) {
        use gtk::glib::spawn_future_local;
        use webkit6::prelude::*;

        let webview = request.webview.clone();
        spawn_future_local(async move {
            let response = on_request.await;
            let back: String = format!("result,{},{}", id, response);
            webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
    });
} 

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

            let back: String = format!("result,{},{}", id, response);
            webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
    });
} 


#[cfg(target_os = "windows")]
pub fn request_blocking<F: FnOnce() -> String + Send + 'static>(
    request: &Request, id: String, on_request: F) {
        use std::{ffi::c_void, thread};
        use webview2_com::CoTaskMemPWSTR;
        use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
        use windows::Win32::UI::WindowsAndMessaging::PostMessageW;

        use crate::windows::webview::WM_SENDRESPONSE;

        let hwnd = request.hwnd;

        thread::spawn(move|| {
            let response = on_request();
            let back: String = format!("result,{},{}", id, response);
            let mut back = CoTaskMemPWSTR::from(back.as_str());
            let wparam: WPARAM = WPARAM(back.take().as_ptr() as usize); 
            let lparam: LPARAM = LPARAM(0);   
            let hwnd = hwnd as *mut c_void;
            let hwnd = HWND(hwnd);
            unsafe { PostMessageW(hwnd, WM_SENDRESPONSE, wparam, lparam).unwrap() };
        });
} 
