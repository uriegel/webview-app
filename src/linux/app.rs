use std::env;

use gio::{ActionMapExt, ApplicationExt, ApplicationFlags, SimpleAction, prelude::ApplicationExtManual};
use gtk::{Application, GtkApplicationExt};

use crate::{appsettings::AppSettings, linux::mainwindow::MainWindow};

pub struct App {
    application: Application
}

impl App {
    pub fn new(settings: AppSettings) -> Self {

        let application = Application::new(Some(&settings.application_id), 
        ApplicationFlags::empty())
            .expect("Application::new() failed");

        let action = SimpleAction::new("destroy", None);
        let weak_application = application.clone();
        action.connect_activate(move |_,_| weak_application.quit());
        application.add_action(&action);
        application.set_accels_for_action("app.destroy", &["<Ctrl>Q"]);

        application.connect_startup(move |application| {
            MainWindow::new(application, &settings);
            ()
        });
    
        application.connect_activate(|_| {});
            
        App { application }
    }

    pub fn run(&self) {
        self.application.run(&env::args().collect::<Vec<_>>());
    }
}