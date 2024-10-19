use std::{ffi::OsString, os::windows::ffi::OsStringExt, slice};

use windows::Win32::{Foundation::{HWND, WPARAM}, System::Com::CoTaskMemFree, UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, WINDOW_LONG_PTR_INDEX}};
use windows_core::PWSTR;

pub mod application;
pub mod webview;
mod framewindow;

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongW(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrW(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongW(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrW(window, index)
}

fn string_to_pcwstr(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

fn _string_from_pcwstr_arr(pwcstr: &[u16]) -> String {
    let pstr: Vec<u16> = 
        pwcstr
            .iter()
            .take_while(|&&i|i != 0)
            .cloned()
            .collect();
    String::from_utf16_lossy(&pstr)
}

fn _string_from_pcwstr(pwcstr: *const u16) -> String {
    if pwcstr.is_null() {
        return String::new();
    }
    let mut len = 0;
    unsafe {
        while *pwcstr.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(slice::from_raw_parts(pwcstr, len))
    }
}

fn wparam_to_string_and_free(wparam: WPARAM) -> String {
    unsafe {
        // Cast WPARAM (which contains the pointer to the wide string) to *mut u16 and then wrap in PWSTR
        let ptr: PWSTR = PWSTR(wparam.0 as *mut u16);

        if ptr.is_null() {
            return String::new(); // Handle the case where the pointer is null
        }

        // Find the length of the wide string (null-terminated)
        let mut len = 0;
        while *ptr.0.add(len) != 0 {
            len += 1;
        }

        // Create a wide string slice from the pointer and the length
        let wide_string_slice = std::slice::from_raw_parts(ptr.0, len);

        // Convert the wide string (UTF-16) to an OsString
        let os_string = OsString::from_wide(wide_string_slice);

        // Free the memory allocated for the string using CoTaskMemFree
        CoTaskMemFree(Some(ptr.0 as *mut _));

        // Convert OsString to Rust String (UTF-8)
        os_string.to_string_lossy().to_string()
    }
}