use std::{fs, path::Path};

use gtk::prelude::*;
use adw::Application;

use crate::bounds::Bounds;

use super::mainwindow::MainWindow;

#[derive(Clone)]
pub struct WebView {
    app: Application,
}

impl WebView {
    pub fn new(title: &str, appid: &str, bounds: Bounds, save_bounds: bool, url: &str, _: bool)->WebView {

        let home = std::env::var("HOME").expect("No HOME directory");
        let config_dir = Path::new(&home).join(".config").join(appid);
        if !fs::exists(config_dir.clone()).expect("Could not access local directory") 
            { fs::create_dir(config_dir.clone()).expect("Could not create local directory") } 

        let app = Application::builder()
            .application_id(appid)
            .build();
        let title = title.to_string();
        let url = url.to_string();
        app.connect_activate(move |application| {
            MainWindow::new(application, &config_dir.to_string_lossy(), &title, bounds, save_bounds, &url);
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
