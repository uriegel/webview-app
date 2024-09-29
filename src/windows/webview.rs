// extern crate libloading;

use include_dir::Dir;
use libloading::{Library, Symbol};
use std::{cell::RefCell, fs, path::Path, rc::Rc, slice};

use crate::{bounds::Bounds, content_type, html, params::{Callbacks, Params}};

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct WebView {
    lib: Library,
    //init: Symbol<'a, unsafe extern fn(settings: *const WebViewAppSettings) -> ()>,    
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
            let with_webroot = params.webroot.is_some();
            let webroot = params.webroot.map(|webroot| {
                Rc::new(RefCell::new(webroot))
            });
            let (url, custom_resource_scheme) = match (params.debug_url, with_webroot) {
                (None, true) => (utf_16_null_terminiated("res://webroot/index.html"), true),
                (Some(debug_url), _) => (utf_16_null_terminiated(&debug_url), false),
                (_, _) => (utf_16_null_terminiated(params.url), false)
            };
            let user_data_path = utf_16_null_terminiated(local_path.as_os_str().to_str().expect("user data path invalid"));
            let callback = Box::new(Callback { 
                should_save_bounds: params.save_bounds,
                config_dir: local_path.to_string_lossy().to_string(),
                webroot,
                callbacks: params.callbacks
            });
            let html_ok = utf_16_null_terminiated(html::ok());
            let html_not_found = utf_16_null_terminiated(&html::not_found());
            let settings = WebViewAppSettings { 
                title: title.as_ptr(),
                user_data_path: user_data_path.as_ptr(),
                html_ok: html_ok.as_ptr(),
                html_not_found: html_not_found.as_ptr(),
                x: bounds.x.unwrap_or(-1),
                y: bounds.y.unwrap_or(-1),
                width: bounds.width.unwrap_or(-1),
                height: bounds.height.unwrap_or(-1),
                is_maximized: bounds.is_maximized,
                target: & *callback,
                on_close,
                on_custom_request,
                url: url.as_ptr(),
                without_native_titlebar: params.without_native_titlebar,
                devtools: params.devtools,
                custom_resource_scheme, 
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
    webroot: Option<Rc<RefCell<Dir<'static>>>>,
    callbacks: Callbacks    
}

impl Callback {
    fn on_custom_request(&self, url: *const u16, url_len: u32, result: &mut RequestResult) {
        unsafe {
            let bytes = slice::from_raw_parts(url, url_len as usize);
            let bytes: Vec<u16> = Vec::from(bytes);
            let url = String::from_utf16_lossy(&bytes);
            let mut file = url.clone();
            let path = file.split_off(14);

            match self.webroot.clone().expect("Custom request without webroot").borrow().get_file(path) {
                Some(file)  => {
                    let bytes = file.contents();
                    result.content = bytes.as_ptr();
                    result.len = bytes.len();
                    result.status = 200;
                    let content_type = utf_16_null_terminiated(&content_type::get(&url));
                    let content_type = content_type.as_slice();
                    let mut idx = 0;
                    content_type.iter().for_each(|i|{
                        result.content_type[idx] = *i;
                        idx = idx + 1;
                    });
                },
                None => result.status = 404
            }
        }
    }

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

extern fn on_custom_request(target: *const Callback, url: *const u16, url_len: u32, result: &mut RequestResult) {
    unsafe {
        (*target).on_custom_request(url, url_len, result);
    }
}

extern fn on_close(target: *const Callback, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool { 
    unsafe {
        let res = (*target).on_close(x, y, w, h, is_maximized);
        res
    }
}

#[repr(C)]
struct RequestResult {
    content: *const u8,
    len: usize,
    status: i32,
    content_type: [u16; 100],
}

#[repr(C)]
struct WebViewAppSettings {
    title: *const u16,
    user_data_path: *const u16,
    html_ok: *const u16,
    html_not_found: *const u16,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    is_maximized: bool,
    target: *const Callback,
    on_close: extern fn(target: *const Callback, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool,
    on_custom_request: extern fn(target: *const Callback, url: *const u16, url_len: u32, &mut RequestResult),
    url: *const u16,
    without_native_titlebar: bool,
    custom_resource_scheme: bool,
    devtools: bool,
    default_contextmenu: bool
}

