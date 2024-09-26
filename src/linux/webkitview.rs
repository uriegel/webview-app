use adw::Application;
//use webkit6::ffi::WebKitWebContext;
use webkit6::prelude::*;
use webkit6::WebView;

use super::mainwindow::MainWindow;

pub struct WebkitView {
    //pub _webview: WebView
}

impl WebkitView {
    pub fn new(_application: &Application, mainwindow: MainWindow, url: &str) -> Self {
        let webview = WebView::builder()
            .build();
        mainwindow.window.set_child(Some(&webview));
        webview.load_uri(url);
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