use std::cell::RefCell;

use windows::Win32::{System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED}, UI::HiDpi::{self, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2}};

use crate::webview::WebView;

#[derive(Clone)]
pub struct Application {
    pub appid: String,
    webview: RefCell<Option<WebView>>
}

impl Application {
    pub fn new(appid: &str)->Self {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().unwrap();
        }
        set_process_dpi_awareness();
    
    
        Self {
            appid: appid.to_string(),
            webview: RefCell::new(None)
        }
    }
    
    pub fn get_appid(&self)->String {
        self.appid.clone()
    }

    pub fn on_activate(&self, val: impl Fn()->WebView + 'static) {
        let webview = val();
        *self.webview.borrow_mut() = Some(webview);
    }

    pub fn run(&self)->u32 {
        if let Some(webview) = self.webview.take() {
            webview.webview.run();
        }
        0
    }
}

fn set_process_dpi_awareness()  {
    unsafe { HiDpi::SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).unwrap(); }; 
}
