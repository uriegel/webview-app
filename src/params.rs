use std::rc::Rc;

use include_dir::Dir;

// #[cfg(target_os = "linux")]    
// use adw::Application;
#[cfg(target_os = "linux")]    
use webkit6::WebView;

use crate::{application::Application, bounds::Bounds};

pub struct Params<'a> {
    pub title: &'a str,
    pub app: &'a Application,
    pub bounds: Bounds,
    pub save_bounds: bool,
    pub url: &'a str,
    pub debug_url: Option<String>,
    #[cfg(target_os = "windows")]
    pub without_native_titlebar: bool,
    #[cfg(target_os = "linux")]    
    pub titlebar: Option<Rc<dyn Fn(&adw::Application, &WebView)->gtk::Widget>>,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub webroot: Option<Dir<'static>>
}

