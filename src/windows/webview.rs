// extern crate libloading;

//use core::slice;
use libloading::{Library, Symbol};
use std::{fs, path::Path};

use crate::bounds::Bounds;

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct WebView {
    lib: Library
}

impl WebView {
    pub fn new(title: &str, appid: &str, bounds: Bounds, url: &str, without_native_titlebar: bool)->WebView {
        // TODO path for release dll
        let bytes = include_bytes!("../../WebViewApp/x64/Debug/WebViewApp.dll");
        let path_app = "C:/Projekte/webview-app/WebViewApp.dll";
        // TODO file to user user store
        fs::write(path_app, bytes).expect("Unable to write dll");
        let bytes = include_bytes!("../../WebViewApp/x64/Debug/WebView2Loader.dll");

        // TODO Linux
        #[cfg(unix)]
        let app_data = std::env::var("HOME").expect("No HOME directory");
        let app_data = std::env::var("LOCALAPPDATA").expect("No APP_DATA directory");
        let local_path = Path::new(&app_data).join(appid);
        if !fs::exists(local_path.clone()).expect("Could not access local directory") 
            { fs::create_dir(local_path.clone()).expect("Could not create local directory") } 
        let path = local_path.join("WebView2Loader.dll");
        fs::write(path, bytes).expect("Unable to write dll");

        unsafe {
            let lib = Library::new(path_app).expect("Failed to load DLL");
            let title = utf_16_null_terminiated(title);
            let url = utf_16_null_terminiated(url);
            let local_path = utf_16_null_terminiated(local_path.as_os_str().to_str().expect("user data path invalid"));
            let settings = WebViewAppSettings { 
                title: title.as_ptr(),
                user_data_path: local_path.as_ptr(),
                x: bounds.x.unwrap_or(-1),
                y: bounds.y.unwrap_or(-1),
                width: bounds.width.unwrap_or(-1),
                height: bounds.height.unwrap_or(-1),
                is_maximized: bounds.is_maximized,
                url: url.as_ptr(),
                without_native_titlebar 
            };            
            let init: Symbol<unsafe extern fn(settings: *const WebViewAppSettings) -> ()> = lib.get(b"Init").expect("Failed to load function 'Init'");
            init(&settings as *const WebViewAppSettings);
            WebView { lib }
        }
    }

    pub fn run(&self)->u32 {
        unsafe {
            let run: Symbol<unsafe extern fn() -> u32> = self.lib.get(b"Run").expect("Failed to load function 'Run'");
            let res = run();

            // let func: Symbol<unsafe extern fn(*const u16) -> *const u16> =
            //     self.lib.get(b"Test1").expect("Failed to load function");
            
            // let wc = utf_16_null_terminiated("Das ist ein sehr schöner Messagebox-Text");
            // let text_ptr = func(wc.as_ptr());
            // let strlen: Symbol<unsafe extern fn(*const u16) -> usize> =
            //     self.lib.get(b"Strlen").expect("Failed to load function");
            // let bytes = slice::from_raw_parts(text_ptr, strlen(text_ptr));
            // let bytes: Vec<u16> = Vec::from(bytes);
            // let text = String::from_utf16_lossy(&bytes);
            // let wc = utf_16_null_terminiated(&text);
            // func(wc.as_ptr());
            // let free: Symbol<unsafe extern fn(*const u16) -> ()> =
            //     self.lib.get(b"Free").expect("Failed to load function");
            // free(text_ptr);
            res
        }
    }
}

#[repr(C)]
struct WebViewAppSettings {
    title: *const u16,
    user_data_path: *const u16,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    is_maximized: bool,
    url: *const u16,
    without_native_titlebar: bool
}

