//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.

#[cfg(target_os = "linux")]
use crate::linux::webview::WebView as WebViewImpl;
#[cfg(target_os = "windows")]
use crate::windows::webview::WebView as WebViewImpl;

/// WebView is a Window running as program including a web view
/// 
/// WebView ihas to be built with the help of the ```WebView::builder``` function
pub struct WebView {
    webview: WebViewImpl
}

/// Implementation of WebView
impl WebView {
    /// Runs the web view application.
    /// 
    /// The function blocks until the window (and the application) is closed.
    pub fn run(&self)->u32 {
        self.webview.run()
    }
}

// TODO Trait
/// Builder to construct a WebView
pub struct WebViewBuilder {
    title: Option<String>
}

impl WebView {
    /// Creates a ```WebViewBuilder``` to construct a WebView.
    /// 
    /// Call several WebViewBuilder functions and create the WebView with ```build()```
    pub fn builder()->WebViewBuilder {
        WebViewBuilder { title: None }
    }
}

impl WebViewBuilder {
    /// Builds the WebView.
    /// 
    /// Call this function when all settings are set.
    pub fn build(&self)->WebView {
        let title = self.title.clone().unwrap_or_else(||{String::from("Webview App")});
        WebView { webview: WebViewImpl::new(&title) }
    }

    /// Sets the title of the window containing the web view.
    pub fn title(mut self, val: String)->WebViewBuilder {
        self.title = Some(val);
        self
    }
}

