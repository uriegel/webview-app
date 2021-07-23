use std::{any::Any, sync::{Arc, Mutex}};

use gio::{
    ApplicationFlags, SimpleAction, prelude::ApplicationExtManual, traits::{
        ActionMapExt, ApplicationExt
    }
};
use gtk::{Application, prelude::GtkApplicationExt};
use crate::{app::AppSettings, linux::mainwindow::MainWindow};

#[derive(Clone)]
pub struct App {
    pub settings: AppSettings,
    application: Application
}

impl App {
    pub fn new(settings: AppSettings, state: Arc<Mutex<Box<dyn Any + Send>>>) -> Self {

        let application = Application::new(Some(&settings.application_id), ApplicationFlags::empty());
        let action = SimpleAction::new("destroy", None);
        let weak_application = application.clone();
        action.connect_activate(move |_,_| weak_application.quit());
        application.add_action(&action);
        application.set_accels_for_action("app.destroy", &["<Ctrl>Q"]);

        let settings_clone = settings.clone();
        application.connect_startup(move |application| {
            MainWindow::new(application, &settings_clone, state.clone());
            ()
        });
    
        application.connect_activate(|_| {});

        let settings = settings.clone();
        App { application, settings: settings.clone() }
    }

    pub fn run(&self) {
        self.application.run();
    }
}