use crate::webview::WebView;
#[cfg(target_os = "linux")]
use crate::linux::application::Application as ApplicationImpl;
#[cfg(target_os = "windows")]
use crate::windows::application::Application as ApplicationImpl;

/// The Application represents a Windows or Linux Gtk Application running the WebView window
#[derive(Clone)]
pub struct Application {
    pub app: ApplicationImpl
}

impl Application {
    /// Creates an application
    /// 
    /// param appid:
    /// It is a reverse domain name, like "de.uriegel.webapp"
    /// 
    /// On Linux, this is the GTK Application ID.
    /// 
    /// It is also used as path part to a directory to share window settings
    /// 
    /// * Windows: ```$LOCALAPPDATA$/<appid>```
    /// * Linux:  ```~/.config/<appid>```
    /// 
    pub fn new(appid: &str)->Self {
        Application {
            app: ApplicationImpl::new(appid) 
        }
    }

    /// Retrieves the app id set on creation
    pub fn get_appid(&self)->String {
        self.app.get_appid()
    }

    /// when the application is started, this method is being called to give you the oppertunity to 
    /// create the WebView. The callback expects a WebView build with the WebViewBuilder.
    pub fn on_activate(&self, val: impl Fn(&Application)->WebView + 'static)->&Self {
        let app = self.clone();
        self.app.on_activate(move ||{
            val(&app)
        });
        self
    }

    /// Runs the web view application.
    /// 
    /// The function blocks until the window (and the application) is closed.
    /// After calling this function, on_activate callback is being called
    pub fn run(&self)->u32 {
        self.app.run()
    }
}


