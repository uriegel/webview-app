use gtk::prelude::*;
use gtk::Application;
use webkit6::prelude::*;
use webkit6::WebView;

use super::mainwindow::MainWindow;

pub struct WebkitView {
    pub webview: WebView
}

impl WebkitView {
    pub fn new(application: &Application, mainwindow: MainWindow, url: &str) -> Self {
        let webview = WebView::builder()
            .build();
        mainwindow.window.set_child(Some(&webview));
        webview.load_uri(url);
        WebkitView {
            webview
        }
    }



}