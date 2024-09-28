use std::rc::Rc;

use include_dir::Dir;

use crate::bounds::Bounds;

pub struct Params<'a> {
    pub title: &'a str,
    pub appid: &'a str,
    pub bounds: Bounds,
    pub save_bounds: bool,
    pub url: &'a str,
    #[cfg(target_os = "windows")]
    pub without_native_titlebar: bool,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub webroot: Option<Dir<'static>>,                    
    pub callbacks: Callbacks
}

#[derive(Clone)]
pub struct Callbacks {
    pub on_close: Rc<dyn Fn()->bool>
}