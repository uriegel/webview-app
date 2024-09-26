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
    pub fn new(app: &Application, config_dir: &str, title: &str, bounds: Bounds, save_bounds: bool, url: &str)->Self {
        let bounds = 
            if save_bounds
                {Bounds::restore(config_dir).unwrap_or(bounds)} 
            else
                {bounds};

        let window = MainWindow { 
            window: 
                ApplicationWindow::builder()
                    .title(title)
                    .application(app)
                    .default_width(bounds.width.unwrap_or(800))
                    .default_height(bounds.height.unwrap_or(600))
                    .titlebar(&HeaderBar::new())
                    .build()
        };

        WebkitView::new(app, window.clone(), url);
        window.window.present();
        if save_bounds  {
            let gtkwindow = window.window.clone();
            let config_dir = config_dir.to_string();
            window.window.connect_close_request(move|_| {
                let bounds = Bounds {
                    x: None,
                    y: None,
                    width: Some(gtkwindow.width()),
                    height: Some(gtkwindow.height()),
                    is_maximized: gtkwindow.is_maximized()
                };
                bounds.save(&config_dir);
                false.into()
            });
        }
        window
    }
}
