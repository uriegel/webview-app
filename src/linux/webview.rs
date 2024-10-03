use std::{cell::RefCell, fs, path::Path, rc::Rc};

use crate::params::Params;

use super::mainwindow::{MainWindow, MainWindowParams};

#[derive(Clone)]
pub struct WebView {
}

impl WebView {
    pub fn new(params: Params)->Self {
        let home = std::env::var("HOME").expect("No HOME directory");
        let config_dir = Path::new(&home).join(".config").join(params.app.get_appid());
        if !fs::exists(config_dir.clone()).expect("Could not access local directory") 
            { fs::create_dir(config_dir.clone()).expect("Could not create local directory") } 

        let title = params.title.to_string();
        let url = params.url.to_string();
        let debug_url = params.debug_url.map(|s|s.to_string());
        let webroot = params.webroot.map(|webroot| {
            Rc::new(RefCell::new(webroot))
        });
                
        let mainwindow_params = MainWindowParams {
            config_dir: &config_dir.to_string_lossy(),
            app: &params.app.app.app.clone(),
            title: &title,
            bounds: params.bounds, 
            save_bounds: params.save_bounds, 
            url: &url,
            debug_url: debug_url.clone(),
            default_contextmenu: params.default_contextmenu,
            devtools: params.devtools,
            webroot: webroot.clone(),
            on_close: params.callbacks.on_close.clone(),
            titlebar: params.titlebar.clone()
        };
        MainWindow::new(mainwindow_params);
        WebView { 
        }
    }
}
