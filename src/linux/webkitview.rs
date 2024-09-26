use adw::Application;
//use webkit6::ffi::WebKitWebContext;
use webkit6::prelude::*;
use webkit6::WebView;

use super::mainwindow::MainWindow;

pub struct WebkitView {
    //pub _webview: WebView
}

pub struct WebkitViewParams<'a> {
    pub _application: &'a Application, 
    pub mainwindow: MainWindow, 
    pub url: &'a str,
    pub devtools: bool,
    pub default_contextmenu: bool
}

impl WebkitView {
    pub fn new(params: WebkitViewParams) -> Self {
        let webview = WebView::builder()
            .build();
        params.mainwindow.window.set_child(Some(&webview));
        if params.devtools {
            let settings = webkit6::prelude::WebViewExt::settings(&webview);
            settings.unwrap().set_enable_developer_extras(true);
        }
        if !params.default_contextmenu {
            webview.connect_context_menu(|_,_,_|true);
        }

        //let a = webview.context().unwrap();
        // a.register_uri_scheme("res", | aa | {
        //     aa.
        // });
        webview.load_uri(params.url);

        WebkitView {
            //webview
        }
        
    }
}