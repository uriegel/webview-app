//! This module contains all the important structs and implementations to create, configure
//! and run an application containing only a webview.
use std::{any::Any, env, net::SocketAddr, path::PathBuf, sync::{Arc, Mutex}};

#[cfg(target_os = "linux")]
use gtk::{Application, ApplicationWindow, Builder};
#[cfg(target_os = "linux")]
use webkit2gtk::WebView;
#[cfg(target_os = "windows")]
use core::marker::PhantomData;

use tokio::runtime::Runtime;

#[cfg(target_os = "linux")]
use crate::linux::app::App as AppImpl;
use crate::warp_server::start;
#[cfg(target_os = "windows")]
use crate::windows::app::App as AppImpl;

#[cfg(target_os = "linux")]
const WEBMSG: &str = "!!webmesg!!";

/// Configuration Settings for the internal warp server.
///
/// Here you have to define the port the warp server is using. You also have the possibility to add filter routes
/// by implementing the "warp_fn".  
pub struct WarpSettings {
    /// port that the internal warp server is using for serving the web files locally.
    pub port: u16, 
    /// If set, then the internal warp server calls this function, and you can manually add routes to warp.
    /// You have to add the static route, which comes by argument, and you also have to start the warp server in this function!
    /// 
    /// # Example:
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    /// use tokio::runtime::Runtime;
    /// use warp::fs::dir;
    /// use webview_app::{app::App, app::{AppSettings, WarpInitData, WarpSettings}, warp_server::add_headers};
    /// use warp::{Filter, reply::{Json, json}};
    /// 
    /// #[derive(Serialize)]
    /// #[serde(rename_all = "camelCase")]
    /// pub struct WarpItem {
    ///     pub name: String,
    ///     pub display: String,
    ///     pub capacity: u64,
    /// }
    /// 
    /// async fn get_item()->Result<Json, warp::Rejection> {
    ///     let item = WarpItem { 
    ///         capacity:123, 
    ///         display: "Warp returning json data".to_string(), 
    ///         name: "warp filter".to_string()
    ///     };
    ///     Ok(json(&item))
    ///     //Err(warp::reject())
    /// }
    /// 
    /// fn server(rt: &Runtime, data: WarpInitData) {
    ///     rt.spawn(async move {
    /// 
    ///         let get_json = 
    ///             warp::get()
    ///             .and(warp::path("requests"))
    ///             .and(warp::path("getitem"))
    ///             .and(warp::path::end())
    ///             .and_then(get_item);
    /// 
    ///         let route_static = dir(data.static_dir)
    ///             .map(add_headers);
    /// 
    ///         let routes = 
    ///             get_json
    ///             .or(route_static);
    /// 
    ///         warp::serve(routes)
    ///             .run(data.socket_addr)
    ///             .await;        
    ///     });
    /// }
    /// 
    /// fn run_app() {
    ///     let app = App::new(
    ///         AppSettings { 
    ///             title: "Rust Web View 👍".to_string(),
    ///             url: "http://localhost:9999/examples/warpfilters.html".to_string(),
    ///             warp_settings: Some(WarpSettings { 
    ///                 port: 9999,
    ///                 init_fn: Some(server),
    ///             }),
    ///             enable_dev_tools: true,
    ///             ..Default::default()
    ///         }
    ///     );
    ///     app.run();
    /// }
    /// 
    /// fn main() {
    ///     run_app();
    /// }    
    /// ```
    pub init_fn: Option<fn(rt: &Runtime, data: WarpInitData)>,
}

impl Clone for WarpSettings {
    fn clone(&self) -> WarpSettings {
        WarpSettings { 
            port: self.port,
            init_fn: self.init_fn            
        }
    }
}

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
    /// If "warp_settings" is set, then the internal warp server is activated and serves locally the web files.
    pub warp_settings: Option<WarpSettings>,
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
    pub window_pos_storage_path: Option<String>,

    // TODO: Windows

    /// If you want to initialize your application or main window, set this callback. It is also needed to inject a
    /// callback if you want to receive msgs from javascript.
    ///
    /// This option is only available on linux
    #[cfg(target_os = "linux")]
    pub on_app_init: Option<fn(data: InitData)>,

    //pub on_msg: Option<fn(application: &Application, window: &)

    /// When set to true, you can configure the main window with a glade xml file. This feature is only
    /// available on windows. It is primarily useful for integrating a header bar to the main window.
    /// The glade file has to be named "main.glade", and it has to be placed in the root directory.
    /// It has to contain a WebKitWebView with the id "webview". The main window has to be 
    /// a "GtkApplicationWindow" and uses the id "window". You can add a WebKitSettings object
    /// to configure for example "enable-developer-extras".
    ///
    /// # Example:
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
            warp_settings: self.warp_settings.clone(),
            use_glade: self.use_glade,
            webroot: self.webroot.clone(),
            window_pos_storage_path: self.window_pos_storage_path.clone(),
            on_app_init: self.on_app_init
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
            warp_settings: None,
            enable_dev_tools: false,
            webroot: "".to_string(),
            on_app_init: None
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
            warp_settings: None,
            webroot: "".to_string()
        }   
    }
}

impl AppSettings {
    /// Gets the url which is used internally for displaying in webview
    pub fn get_url(&self)->String {
        if self.url.len() > 0  {
            self.url.clone()
        } else if let Some(ws) = &self.warp_settings { 
            format!("http://localhost:{}", ws.port).to_string() 
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

/// Data which is provided in the "on_app_init" method of "AppSettings"
pub struct InitData<'a> {
    /// The GTK Application. This is only available on Linux
    #[cfg(target_os = "linux")]
    pub application: &'a Application,
    /// The GTK Application Window. This is only available on Linux
    #[cfg(target_os = "linux")]
    pub window: &'a ApplicationWindow,
    /// The  GTK Builder, if chosen with flag "use_glade". This is only available on Linux
    #[cfg(target_os = "linux")]
    pub builder: &'a Option<Builder>,
    /// The GTK WebView. This is only available on Linux
    #[cfg(target_os = "linux")]
    pub webview: &'a WebView,
    /// Possibility to store arbitrary state. This is only available on Linux
    #[cfg(target_os = "linux")]
    pub state: AppState,
    #[cfg(target_os = "windows")]
    phantom: PhantomData<&'a i32>,
}

/// Data which is provided in the "init_fn" method of "WarpSettings"
pub struct WarpInitData {
    /// The socket address the warp server is using
    pub socket_addr: SocketAddr, 
    /// The static directory warp server is using for serving the web app
    pub static_dir: String, 
    /// Possibility to store arbitrary state. This is only available on Linux
    pub state: AppState
}

pub type AppState = Arc<Mutex<Box<dyn Any + Send>>>;

/// This is the app running a window containig only a webview.
pub struct App {
    app: AppImpl,
    state: AppState
}

impl App {
    /// Constructor to create and configure the app. 
    pub fn new(settings: AppSettings) -> Self {
        let state: AppState = Arc::new(Mutex::new(Box::new(0)));
        App { 
            app: AppImpl::new(settings, state.clone()),
            state
        }
    }

    /// With this method the application is started and running, until the window is closed.
    pub fn run(&self) {
        let warp = if let Some(warp_settings) = &self.app.settings.warp_settings {
            Some((Runtime::new().unwrap(), warp_settings))
        } else {
            None
        };
        if let Some((ref rt, warp_settings)) = warp {
            start(rt, warp_settings.clone(), self.state.clone())
        }

        self.app.run();
    }
}

/// If you want callback from javascript, you can call this method in the "on_app_init" callback.
/// You can then call ```sendMessageToWebView("cmd", "payload")``` 
///
/// This option is only available on linux
#[cfg(target_os = "linux")]
pub fn connect_msg_callback<F: Fn(&str, &str)->() + 'static>(webview: &WebView, on_msg: F) {
    use webkit2gtk::traits::WebViewExt;

    webview.connect_script_dialog(move|_, dialog | {
        let str = dialog.get_message();
        if str.starts_with(WEBMSG) {
            let msg = &str[WEBMSG.len()..];
            if let Some(pos) = msg.find("!!") {
                let cmd = &msg[0..pos];
                let payload = &msg[pos+2..];
                on_msg(cmd, payload);
            }
        }
        true
    });
}