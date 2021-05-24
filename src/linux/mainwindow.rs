use std::cell::RefCell;

use gtk::{
    Application, ApplicationWindow, Builder, GtkApplicationExt, GtkWindowExt, 
    WidgetExt, prelude::BuilderExtManual
};

use crate::{app::AppSettings, settings::WindowPosStorage};

use super::webview::MainWebView;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow
}

impl MainWindow {
    pub fn new(application: &Application, settings: &AppSettings) -> Self {
        let window_pos_storage = match &settings.window_pos_storage_path {
            Some(store) => Some(WindowPosStorage::new(&store)),
            None => None
        };
        let initial_size = if let Some(ref store) = window_pos_storage {
            store.initialize_size(settings.width, settings.height)
        } else {
            (settings.width, settings.height)
        };

        let builder = if settings.use_glade {
            let builder = Builder::new();
            builder.add_from_file("main.glade").unwrap();
            Some(builder)
        } else {
            None
        };

        let window: ApplicationWindow = match builder {
            Some(ref builder) => builder.get_object("window").unwrap(),
            None => ApplicationWindow::new(application) 
        };

        let mainwindow = MainWindow { 
            window: window.clone(),
        };
        window.set_title(&settings.title);
        
        let webview = MainWebView::new(application, mainwindow.clone(), 
            &builder, settings);
        webview.load(&settings.get_url());
        window.set_default_size(initial_size.0, initial_size.1);


        if let Some(store) = window_pos_storage {        
            let wh = RefCell::new((0, 0));
            let weak_window = window.clone();
                window.connect_configure_event(move |_,_| {
                let size = weak_window.get_size();
                let old_wh = wh.replace(size);
                if size.0 != old_wh.0 || size.1 != old_wh.1 {
                    store.save_size((size.0,  size.1));
                }
                false
            });        
        }

        application.add_window(&window);
        window.show_all();
        mainwindow
    }
}

