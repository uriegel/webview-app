use gtk::prelude::*;
use adw::Application;

use crate::bounds::{self, Bounds};

use super::mainwindow::MainWindow;

#[derive(Clone)]
pub struct WebView {
    app: Application,
}

impl WebView {
    pub fn new(title: &str, appid: &str, bounds: Bounds, url: &str, _: bool)->WebView {
        let app = Application::builder()
            .application_id(appid)
            .build();
        let title = title.to_string();
        let url = url.to_string();
        app.connect_activate(move |application| {
            MainWindow::new(application, &title, bounds, &url);
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
