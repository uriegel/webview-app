use adw::Application;
use adw::HeaderBar;
use gtk::prelude::*;
use gtk::ApplicationWindow;

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
        let headerbar = HeaderBar::new();
        window.window.set_titlebar(Some(&headerbar));
        window.window.present();
        window
    }
}
