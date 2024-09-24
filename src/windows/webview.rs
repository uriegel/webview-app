// extern crate libloading;

//use core::slice;
use libloading::{Library, Symbol};
use std::fs;

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct WebView {
    lib: Library
}

impl WebView {
    // TODO Borderless Windows then covert to rust
    // TODO Include a webview with dragzone
    // TODO https://github.com/melak47/BorderlessWindow/blob/main/src/main.cpp

    pub fn new(title: Option<String>)->WebView {
        // TODO path for release dll
        let bytes = include_bytes!("../../WebViewApp/x64/Debug/WebViewApp.dll");
        let path = "C:/Projekte/webview-app/test.dll";
        // TODO file to user user store
        fs::write(path, bytes).expect("Unable to write dll");
        unsafe {
            let lib = Library::new(path).expect("Failed to load DLL");
            let title = utf_16_null_terminiated(&title.unwrap_or(String::new()));
            let settings = WebViewAppSettings { title: title.as_ptr()};            
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
    title: *const u16
}

