//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.

#[cfg(target_os = "linux")]
use crate::linux::app::App as AppImpl;
#[cfg(target_os = "windows")]
use crate::windows::webview::WebView as WebViewImpl;

pub struct WebView {
    webview: WebViewImpl
}

impl WebView {
    pub fn run(&self)->u32 {
        self.webview.run()
    }
}

// TODO Trait
pub struct WebViewBuilder {

}

impl WebView {
    pub fn builder()->WebViewBuilder {
        WebViewBuilder {}
    }
}

impl WebViewBuilder {
    pub fn build(&self)->WebView {
        WebView { webview: WebViewImpl::new() }
    }
}

