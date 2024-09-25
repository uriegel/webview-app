use gtk::prelude::*;
use adw::Application;

use super::mainwindow::MainWindow;

#[derive(Clone)]
pub struct WebView {
    app: Application,
}

impl WebView {
    pub fn new(title: &str, appid: &str, url: &str, _: bool)->WebView {
        let app = Application::builder()
            .application_id(appid)
            .build();
        let title = String::from(title);
        let url = url.to_string();
        app.connect_activate(move |application| {
            MainWindow::new(application, &title, &url);
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
