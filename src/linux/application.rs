use gtk::prelude::*;

use crate::webview::WebView;

#[derive(Clone)]
pub struct Application {
    pub app: adw::Application
}

impl Application {
    pub fn new(appid: &str)->Self {
        let application = adw::Application::builder()
            .application_id(appid)
            .build();
        Self {
            app: application.clone()
        }
    }
    
    pub fn get_appid(&self)->String {
        self.app.application_id().map(|str| { 
            str.as_str().to_string()
        }).unwrap_or("de.uriegel.webviewapp".to_string())
    }

    pub fn on_activate(&self, val: impl Fn()->WebView + 'static) {
        self.app.connect_activate(move |_| {
            val();
        });
    }

    pub fn run(&self)->u32 {
        self.app
            .run()
            .value() as  u32
    }
}
