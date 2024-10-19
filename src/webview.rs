//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.

use std::sync::{Arc, Mutex};
#[cfg(target_os = "linux")]
use std::rc::Rc;

use include_dir::Dir;

use crate::{application::Application, bounds::Bounds, params::Params, request::Request, windows::webview::WebViewHandle as WebViewHandleImpl};

#[cfg(target_os = "linux")]
use crate::linux::webview::WebView as WebViewImpl;
#[cfg(target_os = "windows")]
use crate::windows::webview::WebView as WebViewImpl;

/// WebView is a Window running as program including a web view
/// 
/// WebView has to be built with the help of the ```WebView::builder``` function
#[derive(Clone)]
pub struct WebView {
    pub(crate) webview: WebViewImpl
}

#[derive(Clone)]
pub struct WebViewHandle {
    pub(crate) handle: WebViewHandleImpl
}


impl WebView {
    /// Creates a ```WebViewBuilder``` to construct a WebView.
    /// 
    /// Call several WebViewBuilder functions and create the WebView with ```build()```
    /// 
    /// # Example
    /// 
    /// ```
    /// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
    /// Allows console to show up in debug build but not release build.
    ///
    /// use webview_app::webview::WebView;
    /// 
    /// fn main() {
    ///     let webview = 
    ///         WebView::builder()
    ///             .appid("de.uriegel.hello".to_string())
    ///             .title("Rust Web View 👍".to_string())
    ///             .url("https://crates.io/crates/webview_app".to_string())
    ///             .build();
    ///     webview.run();
    /// }
    /// ```
    pub fn builder(app: &Application)->WebViewBuilder {
            WebViewBuilder { 
            title: None,
            app: app.clone(),
            url: None,
            debug_url: None,
            width: None,
            height: None,
            save_bounds: false,
            #[cfg(target_os = "linux")]
            builder_path: None,
            #[cfg(target_os = "linux")]
            with_builder: None,
            without_native_titlebar: false,
            devtools: false,
            default_contextmenu : true,
            webroot: None,
        }
    }

    /// Sets a callback which is invoked an closing the app
    /// 
    /// You can prevent closing the app when returning false
    pub fn can_close(&self, val: impl Fn()->bool + 'static) {
        self.webview.can_close(val);
    }

    pub fn connect_request<F: Fn(&Request, String, String, String) -> bool + 'static>(
        &self,
        on_request: F,
    ) {
        self.webview.connect_request(on_request);
    }   

    pub fn get_handle(&self)->WebViewHandle {
        WebViewHandle {
            handle: self.webview.get_handle()
        }
    }

    pub fn eval(handle: WebViewHandle, script: &str) {
        WebViewImpl::start_evaluate_script(handle, script);
    }
}

/// Builder to construct a WebView
pub struct WebViewBuilder {
    title: Option<String>,
    app: Application,
    url: Option<String>,
    debug_url: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    save_bounds: bool,
    #[cfg(target_os = "linux")]    
    with_builder: Option<Rc<dyn Fn(&gtk::Builder)>>,
    #[cfg(target_os = "linux")]    
    builder_path: Option<String>,
    without_native_titlebar: bool,
    devtools: bool,
    default_contextmenu: bool,
    webroot: Option<Dir<'static>>,
}

impl WebViewBuilder {
    /// Builds the WebView.
    /// 
    /// Call this function when all settings are set.
    pub fn build(self)->WebView {
        let title = self.title;

        let bounds = Bounds {
            x: None,
            y: None,
            width: self.width, 
            height: self.height, 
            is_maximized: false
        };

        let webroot = self.webroot.map(|webroot| Arc::new(Mutex::new(webroot)));

        let params = Params {
            title,
            app: &self.app,
            bounds,
            save_bounds: self.save_bounds,
            url: self.url.clone(),
            debug_url: self.debug_url.clone(),
            #[cfg(target_os = "windows")]
            without_native_titlebar: self.without_native_titlebar,
            devtools: self.devtools,
            default_contextmenu: self.default_contextmenu,
            webroot,
            #[cfg(target_os = "linux")]
            builder_path: self.builder_path,
            #[cfg(target_os = "linux")]#[cfg(target_os = "linux")]
            with_builder: self.with_builder
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
    /// You can use 
    /// * ```http(s)://``` 
    /// * ```file://```
    pub fn url(mut self, val: String)->WebViewBuilder {
        self.url = Some(val);
        self
    }

    #[cfg(target_os = "linux")]
    /// Callback to create the app via a ui resource
    pub fn with_builder(mut self, builder_path: String, on_build: impl Fn(&gtk::Builder) + 'static)->WebViewBuilder {
        self.builder_path = Some(builder_path);
        self.with_builder = Some(Rc::new(on_build));
        self
    }        

    /// Sets the web view's url when debugging
    /// 
    /// This url is used when the app is being debugged. For example, if you use a react website you can set
    /// 
    /// * ```debug_url(http://localhost:5173```)
    /// 
    /// and for the release version you set an url to the published web site
    /// 
    /// You can use 
    /// * ```http(s)://``` 
    /// * ```file://```
    pub fn debug_url(mut self, val: String)->WebViewBuilder {
        if cfg!(debug_assertions) {
            self.debug_url = Some(val);
        }
        self
    }

    /// If you want your web site be included as a resource in the binary file, call this method.
    /// 
    /// You must not call the ```url()``` method. It is set automatically to ```res://webroot/index.html```
    /// 
    /// ```index.html``` has to be present in the webroot directory. All dependant web site resources have to be relatively referenced. 
    /// The complete web site can be included.
    /// 
    /// For this purpose you have to add the crate ```https://crates.io/crates/include_dir```.
    /// 
    /// # Hints
    /// * The path to webroot is relative to the crates root directory
    /// * If you set ```debug_url(...)``` then this url is loaded in debug version (for example a react website hosted by vite in comparison to 
    /// the published react website included in the binary)
    /// 
    /// # example
    /// 
    /// ```
    /// use include_dir::{include_dir};
    /// use webview_app::webview::WebView;
    ///
    /// fn main() {
    ///     let webview = 
    ///         WebView::builder()
    ///             .appid("de.uriegel.hello".to_string())
    ///             .title("Website form custom resources 👍".to_string())
    ///             .webroot(include_dir!("webroots/custom_resources"))
    ///             .build();
    ///     webview.run();
    /// }
    /// ```
    pub fn webroot(mut self, webroot: Dir<'static>)->WebViewBuilder {
        self.webroot = Some(webroot);
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

