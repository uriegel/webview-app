#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

extern crate libloading;

use core::slice;
use libloading::{Library, Symbol};
use std::fs;

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

fn main() {
    let bytes = include_bytes!("../WebViewApp/x64/Debug/WebViewApp.dll");
    let path = "C:/Projekte/webview-app/test.dll";
    fs::write(path, bytes).expect("Unable to write dll");

    // TODO path for release dll
    // TODO Macro std::include_str!
    // TODO file to user user store
    // TODO https://stackoverflow.com/questions/46373028/how-to-release-a-beta-version-of-a-crate-for-limited-public-testing
    // TODO Borderless Windows then covert to rust
    // TODO Include a webview with dragzone
    // TODO https://github.com/melak47/BorderlessWindow/blob/main/src/main.cpp
    unsafe {
        let lib = Library::new(path).expect("Failed to load DLL");
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
}
