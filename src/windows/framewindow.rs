use std::{cell::RefCell, rc::Rc};

use windows::Win32::{
    Foundation::{
        HWND, LPARAM, LRESULT, RECT, SIZE, WPARAM
    }, System::LibraryLoader, UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, GetClientRect, RegisterClassW, CW_USEDEFAULT, WM_SIZE, WM_CLOSE, WM_DESTROY, 
        WNDCLASSW, WS_OVERLAPPEDWINDOW
    }
};
use windows_core::w;

use super::webview::WebView;

#[derive(Clone)]
pub struct FrameWindow {
    pub window: Rc<HWND>,
    pub size: Rc<RefCell<SIZE>>,
}

impl FrameWindow {
    pub fn new() -> Self {
        let hwnd = {
            let window_class = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                lpszClassName: w!("WebView"),
                ..Default::default()
            };

            unsafe {
                RegisterClassW(&window_class);

                CreateWindowExW(
                    Default::default(),
                    w!("WebView"),
                    w!("WebView"),
                    WS_OVERLAPPEDWINDOW, // TODO
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    None,
                    None,
                    LibraryLoader::GetModuleHandleW(None).unwrap_or_default(),
                    None,
                )
            }
        };

        FrameWindow {
            window: Rc::new(hwnd.unwrap_or_default()),
            size: Rc::new(RefCell::new(SIZE { cx: 0, cy: 0 })),
        }
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let webview = match WebView::get_window_webview(hwnd) {
            Some(webview) => webview,
            None => return unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
        };
    
        let frame = &webview.frame;
    
        match msg {
            WM_SIZE => {
                let size = get_window_size(hwnd);
                webview.set_size(size.cx, size.cy);
                *frame.size.borrow_mut() = size;
                   LRESULT::default()
            }
    
            WM_CLOSE => {
                unsafe {
                    let _ = DestroyWindow(hwnd);
                }
                LRESULT::default()
            }
    
            WM_DESTROY => {
                webview.terminate();
                LRESULT::default()
            }
    
           _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
       }
    }
    
    fn get_window_size(hwnd: HWND) -> SIZE {
        let mut client_rect = RECT::default();
        let _ = unsafe { GetClientRect(hwnd, &mut client_rect) };
        SIZE {
            cx: client_rect.right - client_rect.left,
            cy: client_rect.bottom - client_rect.top,
        }
    }
    