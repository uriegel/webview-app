use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use super::webkitview::WebkitView;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow
}

impl MainWindow {
    pub fn new(app: &Application, title: &str, url: &str)->Self {
        let window = MainWindow { 
            window: 
                ApplicationWindow::builder()
                .title(title)
                .application(app)
                .default_width(800)
                .default_height(600)
                .build()
        };

        WebkitView::new(app, window.clone(), url);
        window.window.present();
        window
    }
}
