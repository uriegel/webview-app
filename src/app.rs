#[cfg(target_os = "linux")]
use crate::linux::app::App as GtkApp;
#[cfg(target_os = "windows")]
use crate::windows::app::App as WinApp;

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
    pub fn new(application_id: &str) -> Self {
        App { 
            app: GtkApp::new(application_id) 
        }
    }
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        App { 
            app: WinApp::new() 
        }
    }

    pub fn run(&self) {
        self.app.run();
    }
}