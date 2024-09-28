// extern crate libloading;

//use core::slice;
use libloading::{Library, Symbol};
use std::{fs, path::Path};

use crate::{bounds::Bounds, params::{Callbacks, Params}};

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct WebView {
    lib: Library,
    _callback: Box<Callback>
}

impl WebView {
    pub fn new(params: Params)->WebView {
        let bytes = include_bytes!("../../assets/WebViewApp.dll");
        let app_data = std::env::var("LOCALAPPDATA").expect("No APP_DATA directory");
        let local_path = Path::new(&app_data).join(params.appid);
        if !fs::exists(local_path.clone()).expect("Could not access local directory") 
            { fs::create_dir(local_path.clone()).expect("Could not create local directory") } 
        let path_app = local_path.join("WebViewApp.dll");
        fs::write(path_app.clone(), bytes).expect("Unable to write dll");
        let bytes = include_bytes!("../../assets/WebView2Loader.dll");

        let path_loader = local_path.join("WebView2Loader.dll");
        fs::write(path_loader.clone(), bytes).expect("Unable to write dll");

        let bounds = 
            if params.save_bounds
                { Bounds::restore(&local_path.to_string_lossy()).unwrap_or(params.bounds) } 
            else
                { params.bounds};

        unsafe {
            let _lib = Library::new(path_loader).expect("Failed to load loader DLL");
            let lib = Library::new(path_app).expect("Failed to load app DLL");
            let title = utf_16_null_terminiated(params.title);
            let url = match (params.debug_url, params.webroot) {
                (None, Some(_)) => utf_16_null_terminiated("res://webroot/index.html"),
                (Some(debug_url), _) => utf_16_null_terminiated(&debug_url),
                (_, _) => utf_16_null_terminiated(params.url)
            };
            let user_data_path = utf_16_null_terminiated(local_path.as_os_str().to_str().expect("user data path invalid"));
            let callback = Box::new(Callback { 
                should_save_bounds: params.save_bounds,
                config_dir: local_path.to_string_lossy().to_string(),
                callbacks: params.callbacks
            });
            let settings = WebViewAppSettings { 
                title: title.as_ptr(),
                user_data_path: user_data_path.as_ptr(),
                x: bounds.x.unwrap_or(-1),
                y: bounds.y.unwrap_or(-1),
                width: bounds.width.unwrap_or(-1),
                height: bounds.height.unwrap_or(-1),
                is_maximized: bounds.is_maximized,
                target: & *callback,
                on_close,
                url: url.as_ptr(),
                without_native_titlebar: params.without_native_titlebar,
                devtools: params.devtools,
                default_contextmenu: params.default_contextmenu
            };            
            let init: Symbol<unsafe extern fn(settings: *const WebViewAppSettings) -> ()> = lib.get(b"Init").expect("Failed to load function 'Init'");
            init(&settings as *const WebViewAppSettings);
            WebView { lib, _callback: callback }
        }
    }

    pub fn run(&self)->u32 {
        unsafe {
            let run: Symbol<unsafe extern fn() -> u32> = self.lib.get(b"Run").expect("Failed to load function 'Run'");
            let res = run();

            // let func: Symbol<unsafe extern fn(*const u16) -> *const u16> =
            //     self.lib.get(b"Test1").expect("Failed to load function");
            
            // let wc = utf_16_null_terminiated("Das ist ein sehr schöner Messagebox-Text");
            // let text_ptr = func(wc.as_ptr());
            // let strlen: Symbol<unsafe extern fn(*const u16) -> usize> =
            //     self.lib.get(b"Strlen").expect("Failed to load function");
            // let bytes = slice::from_raw_parts(text_ptr, strlen(text_ptr));
            // let bytes: Vec<u16> = Vec::from(bytes);
            // let text = String::from_utf16_lossy(&bytes);
            // let wc = utf_16_null_terminiated(&text);
            // func(wc.as_ptr());
            // let free: Symbol<unsafe extern fn(*const u16) -> ()> =
            //     self.lib.get(b"Free").expect("Failed to load function");
            // free(text_ptr);
            res
        }
    }
}

struct Callback {
    should_save_bounds: bool,
    config_dir: String,
    callbacks: Callbacks    
}

impl Callback {
    fn on_close(&self, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool {
        let can_close = (*self.callbacks.on_close)();
        if can_close && self.should_save_bounds {
            let bounds = Bounds {
                x: Some(x),
                y: Some(y),
                width: Some(w),
                height: Some(h),
                is_maximized 
            };
            bounds.save(&self.config_dir);
        }
        can_close
    }
}

extern fn on_close(target: *const Callback, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool { 
    unsafe {
        let res = (*target).on_close(x, y, w, h, is_maximized);
        res
    }
}

#[repr(C)]
struct WebViewAppSettings {
    title: *const u16,
    user_data_path: *const u16,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    is_maximized: bool,
    target: *const Callback,
    on_close: extern fn(target: *const Callback, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool,
    url: *const u16,
    without_native_titlebar: bool,
    devtools: bool,
    default_contextmenu: bool
}

