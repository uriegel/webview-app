use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]    
use std::rc::Rc;

use include_dir::Dir;

use crate::{application::Application, bounds::Bounds};

pub struct Params<'a> {
    pub title: Option<&'a str>,
    pub app: &'a Application,
    pub bounds: Bounds,
    pub save_bounds: bool,
    pub url: Option<&'a str>,
    pub debug_url: Option<&'a str>,
    pub query_string: Option<&'a str>,
    #[cfg(target_os = "windows")]
    pub without_native_titlebar: bool,
    #[cfg(target_os = "linux")]    
    pub with_builder: Option<Rc<dyn Fn(&gtk::Builder)>>,
    #[cfg(target_os = "linux")]    
    pub builder_path: Option<&'a str>,
    pub devtools: bool,
    pub console_logging: bool,
    pub default_contextmenu: bool,
    pub background_color: Option<(u8, u8, u8, u8)>,
    pub webroot: Option<Arc<Mutex<Dir<'static>>>>,
}
