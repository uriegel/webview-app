use core::any::Any;
use std::sync::Arc;
use std::sync::Mutex;
use std::{mem, ptr, rc::Rc};
use once_cell::sync::OnceCell;
use winapi::{shared::{windef::{
            DPI_AWARENESS_CONTEXT, 
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
        }
    }, 
    um::{
        libloaderapi::{
            GetModuleHandleW, GetProcAddress, LoadLibraryA
        }, wingdi::{
            GetDeviceCaps, LOGPIXELSX
        }, winuser::{DispatchMessageW, GetDC, GetMessageW, MSG, ReleaseDC, TranslateMessage}
    }
};

use crate::app::AppSettings;

use super::{mainwindow::MainWindow, messagebox::error_message_box};

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct App {
    pub settings: AppSettings
//    main_window: Rc<OnceCell<MainWindow>>
}

impl App {
    pub fn new(settings: AppSettings, _: AppState) -> Self {
        let instance = unsafe { GetModuleHandleW(ptr::null()) };
        App::set_dpi_aware();
        let hdc = unsafe { GetDC(ptr::null_mut()) };
        let dpi = unsafe { GetDeviceCaps(hdc, LOGPIXELSX) };
        unsafe { ReleaseDC(ptr::null_mut(), hdc) };
        
        let main_window = MainWindow::new(&settings);
        let main_window_cell = Rc::new(OnceCell::new());
        match main_window_cell.set(main_window) {
            Ok(()) => (),
            _ => App::error_message_box("Not good")
        }

        let main_window = main_window_cell.get().expect("Main window not initialized");
        main_window.register(instance, &main_window_cell);
        main_window.create(instance, dpi, &settings);        

        App { 
            settings 
            // main_window: main_window_cell 
        }
    }

    pub fn run(&self) {
        let mut msg: MSG = unsafe { mem::zeroed() };
        while unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) } > 0 {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }    
    }

    pub fn error_message_box(text: &str) {
        error_message_box(ptr::null_mut(), text);
    }

    // pub fn message_box(text: &str, mb_type: u32) -> i32 {
    //     message_box(ptr::null_mut(), text, mb_type)
    // }
    
    fn set_dpi_aware() {
        unsafe {
            let user32 = LoadLibraryA(b"user32.dll\0".as_ptr() as *const i8);
            let set_thread_dpi_awareness_context = GetProcAddress(
                user32,
                b"SetThreadDpiAwarenessContext\0".as_ptr() as *const i8,
            );
            if !set_thread_dpi_awareness_context.is_null() {
                let set_thread_dpi_awareness_context: extern "system" fn(
                    DPI_AWARENESS_CONTEXT,
                )
                    -> DPI_AWARENESS_CONTEXT = mem::transmute(set_thread_dpi_awareness_context);
                set_thread_dpi_awareness_context(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
                return;
            }
        }
    }
}

