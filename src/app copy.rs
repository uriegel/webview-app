//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.
use std::{env, io::Error, path::PathBuf, pin::Pin};

use tokio::runtime::Runtime;
use futures::{Future, future};

#[cfg(target_os = "linux")]
use crate::linux::app::App as AppImpl;
use crate::warp_server::start;
#[cfg(target_os = "windows")]
use crate::windows::app::App as AppImpl;

// fn test() -> Pin<Box<dyn Future<Output = Result<bool, ()>, Item = bool, Error = Error>>> {
//     Box::pin(future::ok(true))
// }

// async fn async_fn() -> bool {
//     test().await.unwrap()
// }

// fn main(){
//     async_fn();
//     println!("Hello!");
// }

//trait FilterGetFn { async fn call(&self, args: String) -> warp::Reply; }
//type GetFilterCallback = async fn(param: GetItems)->Result<impl warp::Reply, warp::Rejection>; 
//type GetFilterCallback = async fn(param: GetItems)->Result<impl warp::Reply, warp::Rejection>; 

// trait FilterGetFn { fn call(&self, args: String) -> BoxFuture<'static, warp::Reply, warp::Rejection>; }
// trait FilterGetFn { fn call(&self, args: String) -> BoxFuture<'static, warp::Reply, warp::Rejection>; }


// impl<T, F> FilterGetFn for T
// where 
//     T: Fn(String)->F,
//     F: Future<Output = Result<impl warp::Reply, warp::Rejection>> + 'static,
// {
//     fn call (&self, args: String)->BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
//         Box::pin(self(args))
//     }
// }
// struct WarpGetFilter<GetCb, F>
// where
//     GetCb: Fn(String)->F,
//     F: Future,
// {
//     path: String,
//     callback: GetCb
// }

// type filters = Vec<WarpGetFilter<GetCb, F>
// where
//     GetCb: Fn(String)->F,
//     F: Future>;

/// Configuration settings for the app. 
///
/// Many of the fields are optional, there is a default implementation for this struct.
/// For example, when only the windows title and the url is wanted to be set:
///
/// ```
/// let app = App::new(
///     AppSettings {
///         title: "Rust Web View".to_string(),
///         url: "https://crates.io".to_string(), 
///         ..Default::default()
///     });
/// ```
pub struct AppSettings {
    /// The applicationId used for Gtk Application. Only be used and only is available on Linux
    #[cfg(target_os = "linux")]
    pub application_id: String,
    /// Window title
    pub title: String,
    /// The url the application's webview will use to display its content. If not set, index.html in the root directory will be used,
    /// or http://localhost:{port}, when using integrated warp web server
    pub url: String,
    /// If webroot is set, then the local web files are searched not in the rust project root but in this relative path "webroot". "webroot"
    /// is relative to the root project directory
    pub webroot: String,
    /// If "warp_port" is set, then the internal warp server is activated and serves locally the web files.
    pub warp_port: u16,
    /// Window width in pixel, if "window_pos_storage_path" is not set, otherwise initial window width
    pub width: i32,
    /// Window height in pixel, if "window_pos_storage_path" is not set, otherwise initial window height
    pub height: i32,
    /// If set to "true", the web view develeoper tools can be activated by shortcut "Ctrl+Shift+I".
    /// There is a default action on Linux to show the developer tools: "app.devtools". It can be connected with a
    /// GtkModelButton in a menu or in the HeaderBar. When using the option "use_glade" and you have
    /// inserted a WebKitSettings object, then you have to enable "developer tools" there.
    pub enable_dev_tools: bool,
    /// If set, then window size is automatically saved to a folder with relative path set to "window_pos_storage_path"
    //pub warp_get_filters: WarpGetFilter<>
    pub window_pos_storage_path: Option<String>,
    /// When set to true, you can configure the main window with a glade xml file. This feature is only
    /// available on windows. It is primarily useful for integrating a header bar to the main window.
    /// The glade file has to be named "main.glade", and it has to be placed in the root directory.
    /// It has to contain a WebKitWebView with the id "webview". The main window has to be 
    /// a "GtkApplicationWindow" and uses the id "window". You can add a WebKitSettings object
    /// to configure for example "enable-developer-extras"
    /// Example:
    ///
    /// ```
    ///<?xml version="1.0" encoding="UTF-8"?>
    ///<!-- Generated with glade 3.38.2 -->
    ///<interface>
    ///  <requires lib="gtk+" version="3.24"/>
    ///  <requires lib="webkit2gtk" version="2.28"/>
    ///  <object class="GtkPopoverMenu" id="menu">
    ///    <property name="can-focus">False</property>
    ///    <property name="position">bottom</property>
    ///    <child>
    ///      <object class="GtkBox">
    ///        <property name="visible">True</property>
    ///        <property name="can-focus">False</property>
    ///        <property name="orientation">vertical</property>
    ///        <child>
    ///          <object class="GtkModelButton">
    ///            <property name="visible">True</property>
    ///            <property name="can-focus">True</property>
    ///            <property name="receives-default">True</property>
    ///            <property name="action-name">app.devtools</property>
    ///            <property name="text" translatable="yes">_Show dev tools</property>
    ///          </object>
    ///          <packing>
    ///            <property name="expand">False</property>
    ///            <property name="fill">True</property>
    ///            <property name="position">0</property>
    ///          </packing>
    ///        </child>
    ///      </object>
    ///      <packing>
    ///        <property name="submenu">main</property>
    ///        <property name="position">1</property>
    ///      </packing>
    ///    </child>
    ///  </object>
    ///  <object class="WebKitSettings" type-func="webkit_settings_get_type" id="settings">
    ///    <property name="enable-developer-extras">True</property>
    ///    <property name="user-agent">Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15</property>
    ///    <property name="hardware-acceleration-policy">always</property>
    ///  </object>
    ///  <object class="GtkApplicationWindow" id="window">
    ///    <property name="can-focus">False</property>
    ///    <child>
    ///      <object class="WebKitWebView" type-func="webkit_web_view_get_type" id="webview">
    ///        <property name="visible">True</property>
    ///        <property name="can-focus">True</property>
    ///        <property name="settings">settings</property>
    ///        <child>
    ///          <placeholder/>
    ///        </child>
    ///      </object>
    ///    </child>
    ///    <child type="titlebar">
    ///      <object class="GtkHeaderBar" id="headerbar">
    ///        <property name="visible">True</property>
    ///        <property name="can-focus">False</property>
    ///        <property name="title" translatable="yes">Rust Web View</property>
    ///        <property name="show-close-button">True</property>
    ///        <child>
    ///          <object class="GtkMenuButton">
    ///            <property name="visible">True</property>
    ///            <property name="can-focus">True</property>
    ///            <property name="receives-default">True</property>
    ///            <property name="popover">menu</property>
    ///            <child>
    ///              <object class="GtkImage">
    ///                <property name="visible">True</property>
    ///                <property name="can-focus">False</property>
    ///                <property name="icon-name">open-menu-symbolic</property>
    ///              </object>
    ///            </child>
    ///          </object>
    ///          <packing>
    ///            <property name="pack-type">end</property>
    ///          </packing>
    ///        </child>
    ///      </object>
    ///    </child>
    ///  </object>
    ///</interface>
    ///```    
    #[cfg(target_os = "linux")]
    pub use_glade: bool,
}

#[cfg(target_os = "linux")]
impl Clone for AppSettings {
    fn clone(&self) -> AppSettings {
        AppSettings { 
            application_id: self.application_id.clone(),
            enable_dev_tools: self.enable_dev_tools,
            height: self.height,
            width: self.width,
            title: self.title.clone(),
            url: self.url.clone(),
            use_glade: self.use_glade,
            warp_port: self.warp_port,
            webroot: self.webroot.clone(),
            window_pos_storage_path: self.window_pos_storage_path.clone()
        }
    }
}

#[cfg(target_os = "linux")]
impl Default for AppSettings {
    fn default()->Self { 
        Self {
            application_id: "de.uriegel.webapp".to_string(),
            width: 800,
            height: 600,
            window_pos_storage_path: None,
            title: "".to_string(),
            url: "".to_string(),
            use_glade: false,
            enable_dev_tools: false,
            warp_port: 0,
            webroot: "".to_string()
        }   
    }
}

#[cfg(target_os = "windows")]
impl Default for AppSettings {
    fn default()->Self { 
        Self {
            width: 800,
            height: 600,
            window_pos_storage_path: None,
            title: "".to_string(),
            url: "".to_string(),
            enable_dev_tools: false,
            warp_port: 0,
            webroot: "".to_string()
        }   
    }
}

impl AppSettings {
    /// Gets the url which is used internally for displaying in webview
    pub fn get_url(&self)->String {
        if self.url.len() > 0  {
            self.url.clone()
        } else if self.warp_port != 0 { 
            format!("http://localhost:{}", self.warp_port).to_string() 
        } else { 
            let dir: PathBuf = [ 
                env::current_dir().unwrap().to_str().unwrap(), 
                &self.webroot,
                "index.html"
            ].iter().collect();
            format!("file://{}", dir.to_str().unwrap()).to_string() 
        }
    }
}

/// This is the app running a window containig only a webview.
pub struct App {
    app: AppImpl
}

impl App {
    /// Constructor to create and configure the app. 
    pub fn new(settings: AppSettings) -> Self {
        App { 
            app: AppImpl::new(settings) 
        }
    }

    /// With this method the application is started and running, until the window is closed.
    pub fn run(&self) {
        let rt = if self.app.settings.warp_port > 0 {
            Some(Runtime::new().unwrap())
        } else {
            None
        };
        if let Some(ref rt) = rt {
            start(rt, self.app.settings.warp_port)
        }

        self.app.run();
    }
}

