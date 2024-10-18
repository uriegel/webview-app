use std::{cell::RefCell, rc::Rc};

use webview2_com::CoTaskMemPWSTR;
use windows::Win32::{
    Foundation::{
        HWND, LPARAM, LRESULT, RECT, SIZE, TRUE, WPARAM
    }, System::LibraryLoader, UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, GetClientRect, GetWindowRect, IsZoomed, RegisterClassW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, 
        NCCALCSIZE_PARAMS, SIZE_MAXIMIZED, WM_CLOSE, WM_DESTROY, WM_NCCALCSIZE, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW
    }
};
use windows_core::w;

use crate::bounds::Bounds;

use super::{webview::{WebView, WM_SENDRESPONSE, WM_SENDSCRIPT}, wparam_to_string_and_free};

#[derive(Clone)]
pub struct FrameWindow {
    pub window: Rc<HWND>,
    pub size: Rc<RefCell<SIZE>>
}

impl FrameWindow {
    pub fn new(title: &str, bounds: Bounds) -> Self {
        let hwnd = {
            let window_class = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                lpszClassName: w!("$$WebView_APP$$"),
                style: CS_HREDRAW | CS_VREDRAW,
                ..Default::default()
            };

            unsafe {
                RegisterClassW(&window_class);
                let title = CoTaskMemPWSTR::from(title);
                CreateWindowExW(
                    Default::default(),
                    w!("$$WebView_APP$$"),
                    *title.as_ref().as_pcwstr(),
                    WS_OVERLAPPEDWINDOW, 
                    bounds.x.unwrap_or(CW_USEDEFAULT),
                    bounds.y.unwrap_or(CW_USEDEFAULT),
                    bounds.width.unwrap_or(CW_USEDEFAULT),
                    bounds.height.unwrap_or(CW_USEDEFAULT),
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

    pub fn get_hwnd(&self)->isize {
        self.window.0 as isize
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
                webview.set_size(size.cx, size.cy, w_param == WPARAM(SIZE_MAXIMIZED as usize));
                *frame.size.borrow_mut() = size;
                   LRESULT::default()
            }

            WM_SENDRESPONSE => {
                webview.send_response(w_param);
                LRESULT::default()
            }

            WM_SENDSCRIPT => {
                let js = wparam_to_string_and_free(w_param);
                webview.eval(&js).unwrap();
                LRESULT::default()
            }

            WM_NCCALCSIZE => {
                unsafe {
                    if webview.without_native_titlebar {
                        let is_zoomed = IsZoomed(hwnd) == TRUE;
                        let is_zoomed_top = if is_zoomed {7} else {0};
                        let is_zoomed_all = if is_zoomed { 3 } else {0};
                        
                        if w_param != WPARAM(0) {
                            let l_param = l_param.0 as usize;
                            let params = &mut *(l_param as *mut NCCALCSIZE_PARAMS);
                            params.rgrc[0].top += 1 + is_zoomed_top;
                            params.rgrc[0].bottom -= 5 + is_zoomed_all;
                            params.rgrc[0].left += 5 + is_zoomed_all;
                            params.rgrc[0].right -= 5 + is_zoomed_all;
                        }
                        else {
                            let l_param = l_param.0 as usize;
                            let params = &mut *(l_param as *mut RECT);
                            params.top += 1 + is_zoomed_top;
                            params.bottom -= 5 + is_zoomed_all;
                            params.left += 5 + is_zoomed_all;
                            params.right -= 5 + is_zoomed_all;
                        }
                        LRESULT::default()
                    } 
                    else {
                        DefWindowProcW(hwnd, msg, w_param, l_param)
                    }
                }
            }
    
            WM_CLOSE => {
                unsafe {
                    let mut rect = RECT::default();
                    let _ = GetWindowRect(hwnd, &mut rect);
                    if webview.on_close(rect.left, rect.top, rect.right-rect.left, rect.bottom - rect.top, IsZoomed(hwnd) == TRUE) {
                        let _ = DestroyWindow(hwnd);
                    }
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
    

