use adw::Application;
use adw::HeaderBar;
use gtk::prelude::*;
use gtk::ApplicationWindow;

use super::webkitview::WebkitView;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow
}

use super::super::bounds::Bounds;

impl MainWindow {
    pub fn new(app: &Application, title: &str, bounds: Bounds, url: &str)->Self {
        let window = MainWindow { 
            window: 
                ApplicationWindow::builder()
                .title(title)
                .application(app)
                .default_width(bounds.width.unwrap_or(800))
                .default_height(bounds.height.unwrap_or(600))
                .build()
        };

        WebkitView::new(app, window.clone(), url);
        let headerbar = HeaderBar::new();
        window.window.set_titlebar(Some(&headerbar));
        window.window.present();
        window
    }
}
