use winapi::{shared::windef::HWND, um::winuser::{MB_ICONERROR, MB_OK, MessageBoxW}};

use super::app::utf_16_null_terminiated;

pub fn error_message_box(hwnd: HWND, text: &str) {
    message_box(hwnd, text, MB_OK | MB_ICONERROR);
}

pub fn message_box(hwnd: HWND, text: &str, mb_type: u32) -> i32 {
    let text = utf_16_null_terminiated(text);
    let caption = utf_16_null_terminiated("Commander");

    unsafe { MessageBoxW(hwnd, text.as_ptr(), caption.as_ptr(), mb_type) }
}
