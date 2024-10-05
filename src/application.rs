use crate::webview::WebView;
#[cfg(target_os = "linux")]
use crate::linux::application::Application as ApplicationImpl;
#[cfg(target_os = "windows")]
use crate::windows::application::Application as ApplicationImpl;

#[derive(Clone)]
pub struct Application {
    pub app: ApplicationImpl
}

/// Implementation of Application
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

    pub fn get_appid(&self)->String {
        self.app.get_appid()
    }

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
    pub fn run(&self)->u32 {
        self.app.run()
    }
}


