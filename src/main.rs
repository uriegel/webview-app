#![windows_subsystem = "windows"]

extern crate libloading;

use libloading::{Library, Symbol};

fn main() {
    unsafe {
        // TODO path for release dll
        // TODO copy release dll to path
        // TODO Load Release dll from resource
        // TODO Macro std::include_bytes!
        // TODO Macro std::include_str!
        let lib = Library::new("C:/Projekte/webview-app/WebViewApp/x64/Debug/WebViewApp.dll").expect("Failed to load DLL");
        let func: Symbol<unsafe extern fn() -> ()> =
            lib.get(b"Test1").expect("Failed to load function");

        // Call the function
        func();
    }
}
