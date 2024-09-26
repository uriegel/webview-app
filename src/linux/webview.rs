use std::{fs, path::Path};

use gtk::prelude::*;
use adw::Application;

use crate::params::Params;

use super::mainwindow::{MainWindow, MainWindowParams};

#[derive(Clone)]
pub struct WebView {
    app: Application,
}

impl WebView {
    pub fn new(params: Params)->WebView {
        let home = std::env::var("HOME").expect("No HOME directory");
        let config_dir = Path::new(&home).join(".config").join(params.appid);
        if !fs::exists(config_dir.clone()).expect("Could not access local directory") 
            { fs::create_dir(config_dir.clone()).expect("Could not create local directory") } 

        let app = Application::builder()
            .application_id(params.appid)
            .build();
        let title = params.title.to_string();
        let url = params.url.to_string();
        app.connect_activate(move |application| {
            let mainwindow_params = MainWindowParams {
                app: application,
                config_dir: &config_dir.to_string_lossy(),
                title: &title,
                bounds: params.bounds, 
                save_bounds: params.save_bounds, 
                url: &url,
                on_close: params.callbacks.on_close.clone()
            };
            MainWindow::new(mainwindow_params);
        });
        WebView { 
            app
        }
    }

    pub fn run(&self)->u32 {
        self.app
            .run()
            .value() as  u32
    }
}
