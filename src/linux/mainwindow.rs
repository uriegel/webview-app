use std::cell::RefCell;
use std::rc::Rc;

use adw::Application;
use adw::HeaderBar;
use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk::Widget;
use include_dir::Dir;
use webkit6::WebView;

use super::webkitview::WebkitView;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow,
    pub webview: WebkitView
}

pub struct MainWindowParams<'a> {
    pub config_dir: &'a str, 
    pub title: &'a str, 
    pub app: &'a adw::Application,
    pub bounds: Bounds, 
    pub save_bounds: bool, 
    pub url: &'a str,
    pub debug_url: Option<String>,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub webroot: Option<Rc<RefCell<Dir<'static>>>>,
    #[cfg(target_os = "linux")]    
    pub titlebar: Option<Rc<dyn Fn(&Application, &WebView)->Widget>>
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

        let webkitview_params = WebkitViewParams {
            url: params.url,
            debug_url: params.debug_url,
            default_contextmenu: params.default_contextmenu,
            devtools: params.devtools,
            webroot: params.webroot
        };
        let webview = WebkitView::new(webkitview_params);

        let window = MainWindow { 
            window: 
                ApplicationWindow::builder()
                    .title(params.title)
                    .application(params.app)
                    .default_width(bounds.width.unwrap_or(800))
                    .default_height(bounds.height.unwrap_or(600))
                    .build(),
                webview: webview.clone()                    
        };

        let headerbar: Widget = 
            match params.titlebar {
                Some(titlebar) => (*titlebar)(&params.app, &webview.webview),
                None => HeaderBar::new().upcast::<Widget>()
            };
        window.window.set_child(Some(&webview.webview));
        window.window.set_titlebar(Some(&headerbar));
        window.window.present();
     
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

    pub fn can_close(self, val: impl Fn()->bool + 'static) {
        self.window.connect_close_request(move|_| (val() == false).into());
    }
}
