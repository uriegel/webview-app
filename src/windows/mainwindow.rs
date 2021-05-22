use std::{mem, ptr, rc::Rc};

use once_cell::sync::OnceCell;
use winapi::{shared::{minwindef::HINSTANCE, windef::{
    HBRUSH, HWND, RECT}}, 
    um::{
        winuser::*,
        winbase::MulDiv, winuser::{COLOR_WINDOW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, 
        IDC_ARROW, LoadCursorW, RegisterClassW, SC_MINIMIZE, SC_RESTORE, SWP_NOACTIVATE, SWP_NOZORDER, SW_SHOW, SetWindowPos, ShowWindow, 
        USER_DEFAULT_SCREEN_DPI, UpdateWindow, WNDCLASSW, WS_OVERLAPPEDWINDOW
    }
}};
use crate::{app::AppSettings, settings::WindowPosStorage, windows::app::App, windows::webview::WebView};

use super::{app::utf_16_null_terminiated};

const CLASS_NAME: &str = "Commander";

pub struct MainWindow {
    //hwnd: HWND,
    webview: WebView,
    window_pos_storage: Option<WindowPosStorage>
}

impl MainWindow {
    pub fn new(settings: &AppSettings) -> Self {
        let window_pos_storage = match &settings.window_pos_storage_path {
            Some(store) => Some(WindowPosStorage::new(&store)),
            None => None
        };
        MainWindow { webview: WebView::new(), window_pos_storage }
    }

    pub fn register(&self, instance: HINSTANCE, main_window_cell: &Rc<OnceCell<MainWindow>>) {
        let main_window_cell_clone = main_window_cell.clone();
        let wnd_proc = move |hwnd, msg, w_param, l_param| {
            match msg {
                WM_SIZE => {
                    if let Some(main_window) = main_window_cell_clone.get() {
                        main_window.on_size(hwnd);       
                    }
                    return 0
                }
                WM_MOVE => {
                    if let Some(main_window) = main_window_cell_clone.get() {
                        main_window.on_move();       
                    }
                    return 0
                }
                // Optimization: don't render the webview when the window is minimized.
                WM_SYSCOMMAND if w_param == SC_MINIMIZE => {
                    if let Some(main_window) = main_window_cell_clone.get() {
                        main_window.set_webview_visible(false);       
                    }
                }
                WM_SYSCOMMAND if w_param == SC_RESTORE => {
                    if let Some(main_window) = main_window_cell_clone.get() {
                        main_window.set_webview_visible(true);       
                    }
                }
                // High DPI support.
                WM_DPICHANGED => unsafe {
                    let rect = *(l_param as *const RECT);
                    SetWindowPos(
                        hwnd,
                        ptr::null_mut(),
                        rect.left,
                        rect.top,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                    return 0
                }
                WM_SETFOCUS => {
                    if let Some(main_window) = main_window_cell_clone.get() {
                        main_window.on_focus();       
                    }
                },
                _ => {},
            }
            unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
        };

        let class_name = utf_16_null_terminiated(CLASS_NAME);
        unsafe {
            let class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
                lpfnWndProc: Some(wnd_proc_helper::as_global_wnd_proc(wnd_proc)),
                lpszClassName: class_name.as_ptr(),
                hInstance: instance,
                hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
                hIcon: LoadIconW(instance, MAKEINTRESOURCEW(1)),
                .. mem::zeroed() 
            };        
            if RegisterClassW(&class) == 0 {
                App::error_message_box(&format!("RegisterClassW failed: {}", std::io::Error::last_os_error()));
                return;
            }
        }
    }

    pub fn create(&self, instance: HINSTANCE, dpi: i32, settings: &AppSettings) {
        let class_name = utf_16_null_terminiated(CLASS_NAME);
        let initial_size = if let Some(ref store) = self.window_pos_storage {
            store.initialize_size(settings.width, settings.height)
        } else {
            (settings.width, settings.height)
        };
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                utf_16_null_terminiated(&settings.title).as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                MulDiv(initial_size.0, dpi, USER_DEFAULT_SCREEN_DPI),
                MulDiv(initial_size.1, dpi, USER_DEFAULT_SCREEN_DPI),
                ptr::null_mut(),
                ptr::null_mut(),
                instance,
                ptr::null_mut(),
            )
        };  
        if hwnd.is_null() {
            App::error_message_box(&format!("CreateWindowExW failed: {}", std::io::Error::last_os_error()));
            return;
        }
        unsafe {
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
        }        
        self.webview.initialize(hwnd, settings.get_url().clone(), settings.enable_dev_tools);
    }

    fn on_size(&self, hwnd: HWND) {
        let mut rect = unsafe { mem::zeroed() };
        unsafe {
            GetClientRect(hwnd, &mut rect);
        }
        self.webview.on_size(rect);
        if let Some(store) = &self.window_pos_storage {
            store.save_size((rect.right, rect.bottom));
        }
    }

    fn on_move(&self) {
        self.webview.on_move();
    }

    fn set_webview_visible(&self, visible: bool) {
        self.webview.set_visible(visible);
    }

    fn on_focus(&self) {
        self.webview.on_focus()
    }
}

mod wnd_proc_helper {
    use winapi::{shared::{minwindef::{LPARAM, UINT, WPARAM}, windef::HWND}, um::winuser::{PostQuitMessage, WM_DESTROY}};

    use super::*;
    use std::cell::UnsafeCell;

    struct UnsafeSyncCell<T> {
        inner: UnsafeCell<T>,
    }

    impl<T> UnsafeSyncCell<T> {
        const fn new(t: T) -> UnsafeSyncCell<T> {
            UnsafeSyncCell {
                inner: UnsafeCell::new(t),
            }
        }
    }

    impl<T: Copy> UnsafeSyncCell<T> {
        unsafe fn get(&self) -> T {
            self.inner.get().read()
        }

        unsafe fn set(&self, v: T) {
            self.inner.get().write(v)
        }
    }

    unsafe impl<T: Copy> Sync for UnsafeSyncCell<T> {}

    static GLOBAL_F: UnsafeSyncCell<usize> = UnsafeSyncCell::new(0);

    /// Use a closure as window procedure.
    ///
    /// The closure will be boxed and stored in a global variable. It will be
    /// released upon WM_DESTROY. (It doesn't get to handle WM_DESTROY.)
    pub unsafe fn as_global_wnd_proc<F: Fn(HWND, UINT, WPARAM, LPARAM) -> isize + 'static>(f: F,) 
        -> unsafe extern "system" fn(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> isize
    {
        let f_ptr = Box::into_raw(Box::new(f));
        GLOBAL_F.set(f_ptr as usize);

        unsafe extern "system" fn wnd_proc<F: Fn(HWND, UINT, WPARAM, LPARAM) -> isize + 'static>(
            hwnd: HWND,
            msg: UINT,
            w_param: WPARAM,
            l_param: LPARAM,
        ) -> isize {
            let f_ptr = GLOBAL_F.get() as *mut F;

            if msg == WM_DESTROY {
                Box::from_raw(f_ptr);
                GLOBAL_F.set(0);
                PostQuitMessage(0);
                return 0;
            }

            if !f_ptr.is_null() {
                let f = &*f_ptr;

                f(hwnd, msg, w_param, l_param)
            } else {
                DefWindowProcW(hwnd, msg, w_param, l_param)
            }
        }

        wnd_proc::<F>
    }
}