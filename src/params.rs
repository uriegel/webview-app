use std::{rc::Rc, sync::{Arc, Mutex}};

use include_dir::Dir;

// #[cfg(target_os = "linux")]    
// use adw::Application;

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
    pub with_builder: Option<Rc<dyn Fn(&gtk::Builder)>>,
    #[cfg(target_os = "linux")]    
    pub builder_path: Option<String>,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub webroot: Option<Arc<Mutex<Dir<'static>>>>,
}

