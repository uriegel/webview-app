use std::{ffi::CString, fs, path::Path, sync::Once};

use super::webview::Callback;

pub type FnInit = extern fn(settings: *const WebViewAppSettings);
pub type FnRun = extern fn()->u32;
//pub type FnExecuteScript = extern fn(*const u16)->bool;
pub type FnPostMessageAsString = extern fn(*const u16);
pub type FnShowDevTools = extern fn();

pub struct RawFuncs {
    pub init: FnInit,
    pub run: FnRun,
//    pub execute_script: FnExecuteScript,
    pub postmessage: FnPostMessageAsString,
//    pub postjson: FnPostMessageAsJson,
    pub show_devtools: FnShowDevTools
}

#[repr(C)]
pub struct RequestResult {
    pub content: *const u8,
    pub len: usize,
    pub status: i32,
    pub content_type: [u16; 100],
}

#[repr(C)]
pub struct WebViewAppSettings {
    pub title: *const u16,
    pub user_data_path: *const u16,
    pub html_ok: *const u16,
    pub html_not_found: *const u16,
    pub init_script: *const u16, 
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_maximized: bool,
    pub target: *const Callback,
    pub on_close: extern fn(target: *const Callback, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool,
    pub on_custom_request: extern fn(target: *const Callback, url: *const u16, url_len: u32, &mut RequestResult),
    pub on_message: extern fn(target: *const Callback, msg: *const u16, msg_len: u32),
    pub url: *const u16,
    pub without_native_titlebar: bool,
    pub custom_resource_scheme: bool,
    pub devtools: bool,
    pub default_contextmenu: bool
}

pub fn load_raw_funcs(appid: &str)->& 'static RawFuncs {
    unsafe {
        INIT.call_once(|| {
            RAWFUNCS = Some(RawFuncs::new(appid));
        });
        RAWFUNCS.as_ref().unwrap()        
    }
}

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn LoadLibraryA(lpFileName: *const i8) -> *const usize;
    pub fn GetProcAddress(hModule: *const usize, lpProcName: *const u8) -> *const usize;
}

static INIT: Once = Once::new();
static mut RAWFUNCS: Option<RawFuncs> = None;

impl RawFuncs {
    pub fn new(appid: &str)->RawFuncs {
        let bytes = include_bytes!("../../assets/WebViewApp.dll");
        let app_data = std::env::var("LOCALAPPDATA").expect("No APP_DATA directory");
        let local_path = Path::new(&app_data).join(appid);
        if !fs::exists(local_path.clone()).expect("Could not access local directory") 
            { fs::create_dir(local_path.clone()).expect("Could not create local directory") } 
        let path_app = local_path.join("WebViewApp.dll");
        fs::write(path_app.clone(), bytes).expect("Unable to write dll");
        let bytes = include_bytes!("../../assets/WebView2Loader.dll");

        let path_loader = local_path.join("WebView2Loader.dll");
        fs::write(path_loader.clone(), bytes).expect("Unable to write dll");
        unsafe {
            let loader_dll = CString::new(path_loader.to_string_lossy().to_string()).unwrap();
            let app_dll = CString::new(path_app.to_string_lossy().to_string()).unwrap();
            let _module = LoadLibraryA(loader_dll.as_ptr());
            let module = LoadLibraryA(app_dll.as_ptr());
            let fnp = GetProcAddress(module, b"Init\0".as_ptr());
            let init = std::mem::transmute::<*const usize, FnInit>(fnp);
            let fnp = GetProcAddress(module, b"Run\0".as_ptr());
            let run = std::mem::transmute::<*const usize, FnRun>(fnp);
//            let fnp = GetProcAddress(module, b"ExecuteScript\0".as_ptr());
  //          let execute_script = std::mem::transmute::<*const usize, FnExecuteScript>(fnp);
            let fnp = GetProcAddress(module, b"PostMessageAsString\0".as_ptr());
            let postmessage = std::mem::transmute::<*const usize, FnPostMessageAsString>(fnp);
            let fnp = GetProcAddress(module, b"ShowDevTools\0".as_ptr());
            let show_devtools = std::mem::transmute::<*const usize, FnShowDevTools>(fnp);
            RawFuncs {
                init,
                run,
 //               execute_script,
                postmessage,
                show_devtools
            }
        }
    }
}