#[cfg(target_os = "linux")]
use crate::linux::application::Application as ApplicationImpl;

pub struct Application<'a> {
    app: ApplicationImpl<'a>
}

/// Implementation of Application
impl<'a> Application<'a> {
    pub fn new(appid: &'a str, on_activate: impl Fn() + 'static)->Self {
        Application {
            app: ApplicationImpl::new(appid, on_activate) 
        }
    }

    /// Runs the web view application.
    /// 
    /// The function blocks until the window (and the application) is closed.
    pub fn run(&self)->u32 {
        self.app.run()
    }
}


