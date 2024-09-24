use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub struct WebView {
    app: Application,
}

impl WebView {
    pub fn new(title: &str, appid: &str, url: &str, _: bool)->WebView {
        // TODO Adwaita
        let app = Application::builder()
            .application_id(appid)
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
