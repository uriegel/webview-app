use std::cell::RefCell;

use gtk::{Application, Builder, GtkApplicationExt, GtkWindowExt, WidgetExt, Window, prelude::BuilderExtManual};

use crate::{appsettings::AppSettings, settings::{initialize_size, save_size}};

use super::webview::MainWebView;

#[derive(Debug, Clone)]
pub struct MainWindow {
    pub window: Window,
//    header_bar: HeaderBar
}

impl MainWindow {
    pub fn new(application: &Application, settings: &AppSettings) -> Self {
        let initial_size = if settings.save_window_pos {
            initialize_size(settings.width, settings.height)
        } else {
            (settings.width, settings.height)
        };

        let builder = Builder::new();
        builder.add_from_file("main.glade").unwrap();
        let window: Window = builder.get_object("mainwindow").unwrap();

        let mainwindow = MainWindow { 
            window: window.clone()
        };

        let webview = MainWebView::new(application, mainwindow.clone());
        webview.load(&settings.url);
        window.set_default_size(initial_size.0, initial_size.1);

        let wh = RefCell::new((0, 0));
        let weak_window = window.clone();
        window.connect_configure_event(move |_,_| {
            let size = weak_window.get_size();
            let old_wh = wh.replace(size);
            if size.0 != old_wh.0 || size.1 != old_wh.1 {
                save_size((size.0,  size.1));
            }
            false
        });        

        application.add_window(&window);
        window.show_all();
        mainwindow
    }
}

