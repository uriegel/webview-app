use gio::{ActionMapExt};
use gtk::{Application, ContainerExt, GtkApplicationExt};
use webkit2gtk::{Settings, WebContext, WebInspectorExt, WebView, WebViewExt};
use webkit2gtk_sys::WebKitSettings;

use super::mainwindow::MainWindow;

// const WEBMSG_TITLE: &str = "!!webmesg-title!!";

pub struct MainWebView {
    webview: WebView,
    //mainwindow: MainWindow
}

impl MainWebView {
    pub fn new(application: &Application, mainwindow: MainWindow) -> Self {
        let context = WebContext::get_default().unwrap();
        let webview = WebView::with_context(&context);
        mainwindow.window.add(&webview);        
        webview.connect_context_menu(|_, _, _, _| true );
        
        webview.set_settings(WebKitSettings {

        })

        let weak_webview = webview.clone();
        let action = gio::SimpleAction::new("devtools", None);
        action.connect_activate(move |_,_| match weak_webview.get_inspector() {
            Some(inspector) => inspector.show(),
            None => println!("Could not show web inspector")
        });
        application.add_action(&action);
        application.set_accels_for_action("app.devtools", &["F12"]);

        MainWebView{ 
            webview, 
            // mainwindow 
        }
    }

    pub fn load(&self, uri: &str) {
        self.webview.load_uri(uri);
    }
}
