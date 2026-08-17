//! This module contains all the important structs and implementations to create, configure
//! and run a webview window.

#[cfg(target_os = "linux")]
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use include_dir::Dir;

use crate::{application::Application, bounds::Bounds, params::Params, request::Request};

#[cfg(target_os = "linux")]
use crate::linux::{
    webkitview::WebViewHandle as WebViewHandleImpl, webview::WebView as WebViewImpl,
};
#[cfg(target_os = "windows")]
use crate::windows::webview::{WebView as WebViewImpl, WebViewHandle as WebViewHandleImpl};

/// WebView is a Window running as program including a web view
///
/// WebView has to be built with the help of the ```WebView::builder``` function
#[derive(Clone)]
pub struct WebView {
    pub(crate) webview: WebViewImpl,
}

/// With the help of this WebViewHandle you can evaluate script in the WebView (via WebView::eval)
///
/// You can retrieve a WebViewHandle via WebView::get_handle
#[derive(Clone)]
pub struct WebViewHandle {
    pub handle: WebViewHandleImpl,
}

impl WebView {
    /// Creates a ```WebViewBuilder``` to construct a WebView.
    ///
    /// Call several WebViewBuilder functions and create the WebView with ```build()```
    ///
    /// # Example
    ///
    /// ```no_run
    /// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
    /// // Allows the console to show up in debug builds but not release builds.
    ///
    /// use webview_app::{application::Application, webview::WebView};
    ///
    /// fn on_activate(app: &Application)->WebView {
    ///     WebView::builder(app)
    ///         .title("Rust Web View")
    ///         .url("https://crates.io/crates/webview_app")
    ///         .build()
    /// }
    ///
    /// fn main() {
    ///     Application::new("de.uriegel.hello").on_activate(on_activate).run();
    /// }
    /// ```
    pub fn builder(app: &Application) -> WebViewBuilder<'_> {
        WebViewBuilder {
            title: None,
            app: app.clone(),
            url: None,
            debug_url: None,
            query_string: None,
            width: None,
            height: None,
            save_bounds: false,
            #[cfg(target_os = "linux")]
            builder_path: None,
            #[cfg(target_os = "linux")]
            with_builder: None,
            without_native_titlebar: false,
            devtools: false,
            console_logging: false,
            default_contextmenu: true,
            background_color: None,
            webroot: None,
        }
    }

    /// Sets a callback which is invoked on closing the app
    ///
    /// You can prevent closing the app when returning false
    ///
    /// # Example
    ///
    /// ```ignore
    /// let can_close = true;
    /// ...
    /// webview.can_close(move ||can_close);
    /// ```
    pub fn can_close(&self, val: impl Fn() -> bool + 'static) {
        self.webview.can_close(val);
    }

    /// When the webview is created, this callback is being called. You can then install WebView requests.
    ///
    /// The Callback function has the following parameters: a request object, the request id, the command id (the Webview method) and the
    /// JSON payload.
    ///
    /// A true return value signals that the request is being processed by this callback.
    pub fn connect_request<F: Fn(&Request, String, String, String) -> bool + 'static>(
        &self,
        on_request: F,
    ) {
        self.webview.connect_request(on_request);
    }

    /// Retrieving a WebViewHandle to evaluate script in the WebView
    pub fn get_handle(&self) -> WebViewHandle {
        WebViewHandle {
            handle: self.webview.get_handle(),
        }
    }

    /// Evaluates script in the WebView
    ///
    /// You need a WebViewHandle which you can retrieve via WebView::get_handle
    ///
    /// You do not need a reference to the WebView handle!
    pub fn eval(handle: WebViewHandle, script: &str) {
        WebViewImpl::start_evaluate_script(handle, script);
    }

    #[cfg(target_os = "windows")]
    /// Execute script in webview
    pub fn execute_javascript(script: &str) {
        WebViewImpl::execute_javascript(script);
    }
}

/// Builder to construct a WebView
pub struct WebViewBuilder<'a> {
    title: Option<&'a str>,
    app: Application,
    url: Option<&'a str>,
    debug_url: Option<&'a str>,
    query_string: Option<&'a str>,
    width: Option<i32>,
    height: Option<i32>,
    save_bounds: bool,
    #[cfg(target_os = "linux")]
    with_builder: Option<Rc<dyn Fn(&gtk::Builder)>>,
    #[cfg(target_os = "linux")]
    builder_path: Option<&'a str>,
    without_native_titlebar: bool,
    devtools: bool,
    console_logging: bool,
    default_contextmenu: bool,
    background_color: Option<(u8, u8, u8, u8)>,
    webroot: Option<Dir<'static>>,
}

impl<'a> WebViewBuilder<'a> {
    /// Builds the WebView.
    ///
    /// Call this function when all settings are set.
    pub fn build(self) -> WebView {
        let bounds = Bounds {
            x: None,
            y: None,
            width: self.width,
            height: self.height,
            is_maximized: false,
        };

        let webroot = self.webroot.map(|webroot| Arc::new(Mutex::new(webroot)));

        let params = Params {
            title: self.title,
            app: &self.app,
            bounds,
            save_bounds: self.save_bounds,
            url: self.url,
            debug_url: self.debug_url,
            query_string: self.query_string,
            #[cfg(target_os = "windows")]
            without_native_titlebar: self.without_native_titlebar,
            devtools: self.devtools,
            console_logging: self.console_logging,
            default_contextmenu: self.default_contextmenu,
            background_color: self.background_color,
            webroot,
            #[cfg(target_os = "linux")]
            builder_path: self.builder_path,
            #[cfg(target_os = "linux")]
            #[cfg(target_os = "linux")]
            with_builder: self.with_builder,
        };

        WebView {
            webview: WebViewImpl::new(params),
        }
    }

    /// Sets the title of the window containing the web view.
    pub fn title(mut self, val: &'a str) -> WebViewBuilder<'a> {
        self.title = Some(val);
        self
    }

    /// With the help of this method you can initialize the size of the window with custom values.
    /// In combination with "save_bounds()" this is the initial width and heigth of the window at first start,
    /// otherwise the window is always starting with these values.
    pub fn initial_bounds(mut self, w: i32, h: i32) -> WebViewBuilder<'a> {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    /// Saves window bounds after closing app.
    ///
    /// When you call save_bounds, then windows location and width and height and normal/maximized state is saved on close.
    /// After restarting the app the webview is displayed at these settings again.
    pub fn save_bounds(mut self) -> WebViewBuilder<'a> {
        self.save_bounds = true;
        self
    }

    /// With the help of this method you can set the initial background color of the webview. For preventing flickering, when
    /// a certain background color is set in HTML, adapt this value here.
    pub fn background_color(
        mut self,
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    ) -> WebViewBuilder<'a> {
        self.background_color = Some((red, green, blue, alpha));
        self
    }

    /// Hides the native window titlebar
    ///
    /// Only working on Windows
    ///
    pub fn without_native_titlebar(mut self) -> WebViewBuilder<'a> {
        self.without_native_titlebar = true;
        self
    }

    #[cfg(target_os = "linux")]
    /// Callback to create the Web View Window via a UI resource
    pub fn with_builder(
        mut self,
        builder_path: &'a str,
        on_build: impl Fn(&gtk::Builder) + 'static,
    ) -> WebViewBuilder<'a> {
        self.builder_path = Some(builder_path);
        self.with_builder = Some(Rc::new(on_build));
        self
    }

    /// Sets the Web View's url
    ///
    /// You can use
    /// * ```http(s)://```
    /// * ```file://```
    pub fn url(mut self, val: &'a str) -> WebViewBuilder<'a> {
        self.url = Some(val);
        self
    }

    /// Sets the Web View's url when debugging
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
    pub fn debug_url(mut self, val: &'a str) -> WebViewBuilder<'a> {
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
    /// ```no_run
    /// use include_dir::{include_dir};
    /// use webview_app::{application::Application, webview::WebView};
    ///
    /// fn on_activate(app: &Application) -> WebView {
    ///     WebView::builder(app)
    ///         .title("Website from custom resources")
    ///         .webroot(include_dir!("$CARGO_MANIFEST_DIR"))
    ///         .build()
    /// }
    ///
    /// fn main() {
    ///     Application::new("de.uriegel.hello").on_activate(on_activate).run();
    /// }
    /// ```
    pub fn webroot(mut self, webroot: Dir<'static>) -> WebViewBuilder<'a> {
        self.webroot = Some(webroot);
        self
    }

    /// Sets the query string to the final webroot's url
    ///
    /// # example
    ///
    /// ```no_run
    /// use include_dir::{include_dir};
    /// use webview_app::{application::Application, webview::WebView};
    ///
    /// fn on_activate(app: &Application)->WebView {
    ///     WebView::builder(app)
    ///             .title("Website from custom resources")
    ///             .webroot(include_dir!("$CARGO_MANIFEST_DIR"))
    ///             .query_string("?param1=test&param2=somthing")
    ///             .build()
    /// }
    ///
    /// fn main() {
    ///     Application::new("de.uriegel.hello").on_activate(on_activate).run();
    /// }
    /// ```
    pub fn query_string(mut self, query_string: &'a str) -> WebViewBuilder<'a> {
        self.query_string = Some(query_string);
        self
    }

    /// Enable (but do not show) the developer tools.
    ///
    /// Used to enable the developer tools. Otherwise it is not possible to open these tools.
    /// The developer tools can be shown by default context menu or by calling the javascript method WebView.showDevtools()
    pub fn devtools(mut self, only_when_debugging: bool) -> WebViewBuilder<'a> {
        self.devtools = true;
        if cfg!(not(debug_assertions)) {
            self.devtools = !only_when_debugging;
        }
        self
    }

    /// Enable or disable writing web console messages to stdout.
    pub fn console_logging(mut self, enabled: bool) -> WebViewBuilder<'a> {
        self.console_logging = enabled;
        self
    }

    /// Disable the default context menu.
    ///
    /// If you set ```default_contextmenu()```, the web view's default context menu is not being displayed when you right click the mouse.
    pub fn default_contextmenu_disabled(mut self) -> WebViewBuilder<'a> {
        self.default_contextmenu = false;
        self
    }
}
