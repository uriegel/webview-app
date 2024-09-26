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
    pub url: &'a str
}

impl WebkitView {
    pub fn new(params: WebkitViewParams) -> Self {
        let webview = WebView::builder()
            .build();
        params.mainwindow.window.set_child(Some(&webview));
        webview.load_uri(params.url);
        let settings = webkit6::prelude::WebViewExt::settings(&webview);
        settings.unwrap().set_enable_developer_extras(true);
        //let a = webview.context().unwrap();
        // a.register_uri_scheme("res", | aa | {
        //     aa.
        // });
        // TODO Disable context menu
        //webview.connect_context_menu(|a, b, c| { true });
        WebkitView {
            //webview
        }
        
    }
}