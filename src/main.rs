#![windows_subsystem = "windows"]

extern crate libloading;

use libloading::{Library, Symbol};
use std::fs;

fn main() {
    unsafe {
        // TODO path for release dll
        // TODO Macro std::include_str!
        // TODO file to user user store

        let bytes = include_bytes!("../WebViewApp/x64/Debug/WebViewApp.dll");
        let path = "C:/Projekte/webview-app/test.dll";
        fs::write(path, bytes).expect("Unable to write dll");
        let lib = Library::new(path).expect("Failed to load DLL");
        let func: Symbol<unsafe extern fn() -> ()> =
            lib.get(b"Test1").expect("Failed to load function");
        func();
    }
}
