use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "de.uriegel.webview_app";

pub struct WebView {
    app: Application,
}

impl WebView {
    pub fn new(title: &str)->WebView {
        // TODO Adwaita
        let app = Application::builder()
            .application_id(APP_ID)
            .build();
        let title = String::from(title);
        app.connect_activate(move |app| {
            // TODO app_id from settings
            let window = ApplicationWindow::builder()
                .title(title.clone())
                .application(app)
                .build();
            window.present();
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
