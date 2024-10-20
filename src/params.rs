use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]    
use std::rc::Rc;

use include_dir::Dir;

use crate::{application::Application, bounds::Bounds};

pub struct Params<'a> {
    pub title: Option<String>,
    pub app: &'a Application,
    pub bounds: Bounds,
    pub save_bounds: bool,
    pub url: Option<String>,
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

