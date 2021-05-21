#[cfg(target_os = "linux")]
use crate::linux::app::App as GtkApp;

#[cfg(target_os = "linux")]
pub struct App {
    app: GtkApp
}

impl App {
    pub fn new(application_id: &str) -> Self {
        App { 
            app: GtkApp::new(application_id) 
        }
    }

    pub fn run(&self) {
        self.app.run();
    }
}