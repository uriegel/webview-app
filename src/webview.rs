//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.

use std::rc::Rc;

use crate::{bounds::Bounds, callbacks::Callbacks};

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
    width: Option<i32>,
    height: Option<i32>,
    save_bounds: bool,
    on_close: Rc<dyn Fn()->bool>,
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
            width: None,
            height: None,
            save_bounds: false,
            on_close: Rc::new(|| true),
            without_native_titlebar: false
        }
    }
}

impl WebViewBuilder {
    /// Builds the WebView.
    /// 
    /// Call this function when all settings are set.
    pub fn build(self)->WebView {
        let title = self.title.clone().unwrap_or_else(||{"Webview App".to_string()});
        let appid = self.appid.clone().unwrap_or_else(||{"de.uriegel.webviewapp".to_string()});
        let url = self.url.clone().unwrap_or_else(||{"about:blank".to_string()});

        let bounds = Bounds {
            x: None,
            y: None,
            width: self.width, 
            height: self.height, 
            is_maximized: false
        };

        let callbacks = Callbacks {
            on_close: self.on_close
        };

        WebView { 
            webview: WebViewImpl::new(&title, &appid, callbacks, bounds, self.save_bounds, &url, self.without_native_titlebar)
        }
    }

    /// Sets the title of the window containing the web view.
    pub fn title(mut self, val: String)->WebViewBuilder {
        self.title = Some(val);
        self
    }

    /// Sets the appid. 
    /// It is a reverse domain name, like "de.uriegel.webapp"
    /// 
    /// On Linux, this is the GTK Application ID.
    /// 
    /// It is also used as path part to a directory to share window settings
    /// 
    /// * Windows: ```$LOCALAPPDATA$/<appid>```
    /// * Linux:  ```~/.config/<appid>```
    pub fn appid(mut self, val: String)->WebViewBuilder {
        self.appid = Some(val);
        self
    }

    /// With the help of this method you can initialize the size of the window with custom values.
    /// In combination with "save_bounds()" this is the initial width and heigth of the window at first start,
    /// otherwise the window is always starting with these values.
    pub fn initial_bounds(mut self, w: i32, h: i32)->WebViewBuilder {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    /// Saves window bounds after closing app.
    /// 
    /// When you call save_bounds, then windows location and width and height and normal/maximized state is saved on close. 
    /// After restarting the app the webview is displayed at these settings again.
    pub fn save_bounds(mut self)->WebViewBuilder {
        self.save_bounds = true;
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
    /// You can use ```http(s)://``` scheme, ```file://``` scheme, and custom resource scheme ```res://```. This value is 
    /// not used, when you set "DebugUrl" and a debugger is attached
    pub fn url(mut self, val: String)->WebViewBuilder {
        self.url = Some(val);
        self
    }

    /// Sets a callback which is invoked an closing the app
    /// 
    /// You can prevent closing the app when returning false
    pub fn can_close(mut self, val: impl Fn()->bool + 'static)->WebViewBuilder {
        self.on_close = Rc::new(val);
        self
    }
}

