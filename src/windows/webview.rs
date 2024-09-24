// extern crate libloading;

use core::slice;
use libloading::{Library, Symbol};
use std::fs;

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct WebView {
}

impl WebView {
    // TODO Borderless Windows then covert to rust
    // TODO Include a webview with dragzone
    // TODO https://github.com/melak47/BorderlessWindow/blob/main/src/main.cpp

    pub fn new()->WebView {
        WebView { }
    }

    pub fn run(&self)->u32 {
        // TODO path for release dll
        let bytes = include_bytes!("../../WebViewApp/x64/Debug/WebViewApp.dll");
        let path = "C:/Projekte/webview-app/test.dll";
        // TODO file to user user store
        fs::write(path, bytes).expect("Unable to write dll");
        unsafe {
            let lib = Library::new(path).expect("Failed to load DLL");
            let init: Symbol<unsafe extern fn(settings: *const WebViewAppSettings) -> ()> = lib.get(b"Init").expect("Failed to load function 'Init'");
            

            let run: Symbol<unsafe extern fn() -> u32> = lib.get(b"Run").expect("Failed to load function 'Run'");
            run();

            let func: Symbol<unsafe extern fn(*const u16) -> *const u16> =
                lib.get(b"Test1").expect("Failed to load function");
            
            let wc = utf_16_null_terminiated("Das ist ein sehr schöner Messagebox-Text");
            let text_ptr = func(wc.as_ptr());
            let strlen: Symbol<unsafe extern fn(*const u16) -> usize> =
                lib.get(b"Strlen").expect("Failed to load function");
            let bytes = slice::from_raw_parts(text_ptr, strlen(text_ptr));
            let bytes: Vec<u16> = Vec::from(bytes);
            let text = String::from_utf16_lossy(&bytes);
            let wc = utf_16_null_terminiated(&text);
            func(wc.as_ptr());
            let free: Symbol<unsafe extern fn(*const u16) -> ()> =
                lib.get(b"Free").expect("Failed to load function");
            free(text_ptr);
        }
        0
    }
}

#[repr(align(8))]
struct WebViewAppSettings {
    title: *const u16
}

