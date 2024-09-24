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

/// Builder to construct a WebView
pub struct WebViewBuilder {
    title: Option<String>,
    appid: Option<String>,
    url: Option<String>,
    without_native_titlebar: bool
}

impl WebView {
    /// Creates a ```WebViewBuilder``` to construct a WebView.
    /// 
    /// Call several WebViewBuilder functions and create the WebView with ```build()```
    pub fn builder()->WebViewBuilder {
        WebViewBuilder { 
            title: None,
            appid: None,
            url: None,
            without_native_titlebar: false
        }
    }
}

impl WebViewBuilder {
    /// Builds the WebView.
    /// 
    /// Call this function when all settings are set.
    pub fn build(&self)->WebView {
        let title = self.title.clone().unwrap_or_else(||{String::from("Webview App")});
        let appid = self.appid.clone().unwrap_or_else(||{String::from("de.uriegel.webviewapp")});
        let url = self.url.clone().unwrap_or_else(||{String::from("about:blank")});
        WebView { webview: WebViewImpl::new(&title, &appid, &url, self.without_native_titlebar) }
    }

    /// Sets the title of the window containing the web view.
    pub fn title(mut self, val: String)->WebViewBuilder {
        self.title = Some(val);
        self
    }

    /// Sets the appid. 
    /// 
    /// On Linux, this is the GTK Application ID.
    /// 
    /// It is also used as path part to a directory to share window settings
    /// 
    /// * Windows: 
    /// * Linux:  ~/
    // TODO docu
    pub fn appid(mut self, val: String)->WebViewBuilder {
        self.appid = Some(val);
        self
    }

    /// Hides the native window titlebar
    /// 
    /// Only working on Windows
    /// 
    pub fn without_native_titlebar(mut self)->WebViewBuilder {
        self.without_native_titlebar = true;
        self
    }

    /// Sets the web view's url
    /// 
    /// There are a few possibilities to show some content, this is one
    pub fn url(mut self, val: String)->WebViewBuilder {
        self.url = Some(val);
        self
    }
}

