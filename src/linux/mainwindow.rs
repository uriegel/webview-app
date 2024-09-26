use std::rc::Rc;

use adw::Application;
use adw::HeaderBar;
use gtk::prelude::*;
use gtk::ApplicationWindow;

use super::webkitview::WebkitView;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow
}

pub struct MainWindowParams<'a> {
    pub app: &'a Application,
    pub config_dir: &'a str, 
    pub title: &'a str, 
    pub bounds: Bounds, 
    pub save_bounds: bool, 
    pub url: &'a str,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub on_close: Rc<dyn Fn()->bool>
}

use super::super::bounds::Bounds;
use super::webkitview::WebkitViewParams;

impl MainWindow {
    pub fn new(params: MainWindowParams)->Self {
        let bounds = 
            if params.save_bounds
                {Bounds::restore(params.config_dir).unwrap_or(params.bounds)} 
            else
                {params.bounds};

        let window = MainWindow { 
            window: 
                ApplicationWindow::builder()
                    .title(params.title)
                    .application(params.app)
                    .default_width(bounds.width.unwrap_or(800))
                    .default_height(bounds.height.unwrap_or(600))
                    .titlebar(&HeaderBar::new())
                    .build()
        };

        let webkitview_params = WebkitViewParams {
            _application: params.app, 
            mainwindow: window.clone(), 
            url: params.url,
            default_contextmenu: params.default_contextmenu,
            devtools: params.devtools
        };

        WebkitView::new(webkitview_params);
        window.window.present();
        window.window.connect_close_request(move|_| ((*params.on_close)() == false).into());
        if params.save_bounds  {
            let gtkwindow = window.window.clone();
            let config_dir = params.config_dir.to_string();
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
