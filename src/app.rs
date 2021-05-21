#[cfg(target_os = "linux")]
use crate::linux::app::App as GtkApp;
#[cfg(target_os = "windows")]
use crate::windows::app::App as WinApp;
use crate::appsettings::AppSettings;

#[cfg(target_os = "linux")]
pub struct App {
    app: GtkApp
}

#[cfg(target_os = "windows")]
pub struct App {
    app: WinApp
}

impl App {
    #[cfg(target_os = "linux")]
    pub fn new(settings: AppSettings) -> Self {
        App { 
            app: GtkApp::new(application_id) 
        }
    }
    #[cfg(target_os = "windows")]
    pub fn new(settings: AppSettings) -> Self {
        App { 
            app: WinApp::new(settings) 
        }
    }

    pub fn run(&self) {
        self.app.run();
    }
}