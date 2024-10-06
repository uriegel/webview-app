use std::{cell::RefCell, fs, path::Path, rc::Rc};

use adw::HeaderBar;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Widget};

use crate::{bounds::Bounds, params::Params, webview::WebView as PubWebView};

use super::webkitview::{WebkitViewParams, WebkitView};

#[derive(Clone)]
pub struct WebView {
    window: ApplicationWindow,
    pub webview: WebkitView
}

impl WebView {
    pub fn new(params: Params)->Self {
        let home = std::env::var("HOME").expect("No HOME directory");
        let config_dir = Path::new(&home).join(".config").join(params.app.get_appid());
        if !fs::exists(config_dir.clone()).expect("Could not access local directory") 
            { fs::create_dir(config_dir.clone()).expect("Could not create local directory") } 
        let config_dir = config_dir.to_string_lossy().to_string();
        let bounds = 
        if params.save_bounds
            {Bounds::restore(&config_dir).unwrap_or(params.bounds)} 
        else
            {params.bounds};

        let debug_url = params.debug_url.map(|s|s.to_string());
        let webroot = params.webroot.map(|webroot| {
            Rc::new(RefCell::new(webroot))
        });

        let webkitview_params = WebkitViewParams {
            url: params.url,
            debug_url: debug_url,
            default_contextmenu: params.default_contextmenu,
            devtools: params.devtools,
            webroot: webroot
        };
        let webview = WebkitView::new(webkitview_params);

        let window = ApplicationWindow::builder()
            .title(params.title)
            .application(&params.app.app.app)
            .default_width(bounds.width.unwrap_or(800))
            .default_height(bounds.height.unwrap_or(600))
            .build();

        let headerbar: Widget = 
            match params.titlebar {
                Some(titlebar) => (*titlebar)(&params.app.app.app, &webview.webview),
                None => HeaderBar::new().upcast::<Widget>()
            };
        window.set_child(Some(&webview.webview));
        window.set_titlebar(Some(&headerbar));
        window.present();           

        if params.save_bounds {
            let gtkwindow = window.clone();
            //let config_dir = config_dir.to_string();
            window.connect_close_request(move|_| {
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
        Self {
            window,
            webview
        }
    }

    pub fn can_close(&self, val: impl Fn()->bool + 'static) {
        self.window.connect_close_request(move|_| (val() == false).into());
    }

    pub fn connect_request<F: Fn(&PubWebView, String, String, String) -> bool + 'static>(
        &self,
        webview: &PubWebView,
        on_request: F,
    ) {
        self.webview.connect_request(webview, on_request);
    }   


}
