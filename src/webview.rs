//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.

use std::{path::{Path, PathBuf}, rc::Rc};

use crate::{bounds::Bounds, params::{Callbacks, Params}};

#[cfg(target_os = "linux")]
use crate::linux::webview::WebView as WebViewImpl;
#[cfg(target_os = "windows")]
use crate::windows::webview::WebView as WebViewImpl;

/// WebView is a Window running as program including a web view
/// 
/// WebView has to be built with the help of the ```WebView::builder``` function
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
    without_native_titlebar: bool,
    devtools: bool,
    default_contextmenu: bool
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
            without_native_titlebar: false,
            devtools: false,
            default_contextmenu : true
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

        let params = Params {
            title: &title,
            appid: &appid,
            bounds,
            save_bounds: self.save_bounds,
            url: &url,
            #[cfg(target_os = "windows")]
            without_native_titlebar: self.without_native_titlebar,
            devtools: self.devtools,
            default_contextmenu: self.default_contextmenu,
            callbacks: Callbacks {
                on_close: self.on_close
            }
        };

        WebView { 
            webview: WebViewImpl::new(params)
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
    /// 
    /// ### Custom resource scheme
    /// 
    /// The complete web site can be included as .NET resources. 
    /// With the ```res://``` url specifier it is possible that the web view is automatically loaded from resources. 
    /// All you have to do is include the website parts
    /// 
    /// The url is in this case:
    /// 
    /// ```
    /// .url("res://index.html".to_string())
    /// ```
    ///  // TODO to be continued...
    pub fn url(mut self, val: String)->WebViewBuilder {
        self.url = Some(val);
        self
    }

    /// If you want your web site be included in the app, call this method.
    /// 
    /// You have to specify the root of your website
    /// 
    /// With the help of the ```url``` method you can load the web site to your webview:
    /// 
    /// ```
    /// res://index.html
    /// ```
    /// 
    /// The webroot path must be relative to the src directory containing this 
    /// // TODO example
    pub fn website_from_resources(mut self, webroot: String)->WebViewBuilder {
        // TODO resolve the website structure in a tree
        // ../webroot/index.html
        // ../webroot/css/script.css
        // TODO use a macro containing include_bytes defined in webview_app and include it in the binary crate
        // ...
        self
    }

    /// Sets a callback which is invoked an closing the app
    /// 
    /// You can prevent closing the app when returning false
    pub fn can_close(mut self, val: impl Fn()->bool + 'static)->WebViewBuilder {
        self.on_close = Rc::new(val);
        self
    }

    /// Enable (not to show) the developer tools.
    /// 
    /// Used to enable the developer tools. Otherwise it is not possible to open these tools. 
    /// The developer tools can be shown by default context menu or by calling the javascript method WebView.showDevtools()
    pub fn devtools(mut self, only_when_debugging: bool)->WebViewBuilder {
        self.devtools = true;
        if cfg!(not(debug_assertions)) {
            self.devtools = !only_when_debugging;
        }
        self
    }

    /// Diables the default context menu.
    /// 
    /// If you set ```default_contextmenu()```, the web view's default context menu is not being displayed when you right click the mouse.    
    pub fn default_contextmenu_disabled(mut self)->WebViewBuilder {
        self.default_contextmenu = false;
        self
    }
}

pub fn get_assets_path(asset_name: &str)->PathBuf {
    let base_path = env!("CARGO_MANIFEST_DIR"); // Get the root directory of the crate
    Path::new(base_path).join("assets").join(asset_name)
}

#[macro_export]
macro_rules! include_here {
    ($x: expr) => {
        include_bytes!($x)
    }
}
