// extern crate libloading;

use include_dir::Dir;
use std::{cell::RefCell, path::Path, rc::Rc, slice};

use crate::{bounds::Bounds, content_type, html, javascript::{self, RequestData}, params::{Callbacks, Params}};

use super::raw_funcs::{load_raw_funcs, RequestResult, WebViewAppSettings};

pub fn utf_16_null_terminiated(x: &str) -> Vec<u16> {
    x.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct WebView {
    _callback: Box<Callback>
}

impl WebView {
    pub fn new(params: Params)->WebView {

        let app_data = std::env::var("LOCALAPPDATA").expect("No APP_DATA directory");
        let local_path = Path::new(&app_data).join(params.app.get_appid());
        let bounds = 
            if params.save_bounds
                { Bounds::restore(&local_path.to_string_lossy()).unwrap_or(params.bounds) } 
            else
                { params.bounds};

        let title = utf_16_null_terminiated(params.title);
        let with_webroot = params.webroot.is_some();
        let webroot = params.webroot.map(|webroot| {
            Rc::new(RefCell::new(webroot))
        });
        let (url, custom_resource_scheme) = match (params.debug_url, with_webroot) {
            (None, true) => (utf_16_null_terminiated("req://webroot/index.html"), true),
            (Some(debug_url), _) => (utf_16_null_terminiated(&debug_url), false),
            (_, _) => (utf_16_null_terminiated(params.url), false)
        };
        let user_data_path = utf_16_null_terminiated(local_path.as_os_str().to_str().expect("user data path invalid"));
        let callback = Box::new(Callback { 
            should_save_bounds: params.save_bounds,
            config_dir: local_path.to_string_lossy().to_string(),
            webroot,
            devtools: params.devtools,
            callbacks: params.callbacks
        });
        let html_ok = utf_16_null_terminiated(html::ok());
        let html_not_found = utf_16_null_terminiated(&html::not_found());
        let script = javascript::get(params.without_native_titlebar, params.title, true, false);
        let init_script = utf_16_null_terminiated(&script);
        let settings = WebViewAppSettings { 
            title: title.as_ptr(),
            user_data_path: user_data_path.as_ptr(),
            html_ok: html_ok.as_ptr(),
            html_not_found: html_not_found.as_ptr(),
            init_script: init_script.as_ptr(),
            x: bounds.x.unwrap_or(-1),
            y: bounds.y.unwrap_or(-1),
            width: bounds.width.unwrap_or(-1),
            height: bounds.height.unwrap_or(-1),
            is_maximized: bounds.is_maximized,
            target: & *callback,
            on_close,
            on_custom_request,
            on_message,
            url: url.as_ptr(),
            without_native_titlebar: params.without_native_titlebar,
            devtools: params.devtools,
            custom_resource_scheme, 
            default_contextmenu: params.default_contextmenu
        };            
        (load_raw_funcs(&params.app.get_appid()).init)(&settings);
        WebView { _callback: callback }
    }

    pub fn run(&self)->u32 {
        (load_raw_funcs("").run)()


        // let run: &'static fn()->u32 = get_function::<fn()->u32>(&self.appid, "Run");
        // let res = run();

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
    }
}

pub struct Callback {
    should_save_bounds: bool,
    devtools: bool,
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

    fn on_message(&self, msg: *const u16, msg_len: u32) {
        unsafe {
            let bytes = slice::from_raw_parts(msg, msg_len as usize);
            let bytes: Vec<u16> = Vec::from(bytes);
            let msg = String::from_utf16_lossy(&bytes);
            if self.devtools && msg == "devtools" 
                { (load_raw_funcs("").show_devtools)() }
            else if msg.starts_with("request,") {
                let request_data = RequestData::new(&msg);
                let back = format!("result,{},{}", request_data.id, request_data.json);
                (load_raw_funcs("").postmessage)(utf_16_null_terminiated(&back).as_ptr()) 
            }
                
        }
    }

}

extern fn on_custom_request(target: *const Callback, url: *const u16, url_len: u32, result: &mut RequestResult) {
    unsafe {
        (*target).on_custom_request(url, url_len, result);
    }
}

extern fn on_message(target: *const Callback, msg: *const u16, msg_len: u32) { 
    unsafe {
       (*target).on_message(msg, msg_len);
    }
}

extern fn on_close(target: *const Callback, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool { 
    unsafe {
        let res = (*target).on_close(x, y, w, h, is_maximized);
        res
    }
}

