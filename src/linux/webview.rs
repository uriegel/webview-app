use gio::{ActionMapExt};
use gtk::{Application, Builder, ContainerExt, GtkApplicationExt, prelude::BuilderExtManual};
use webkit2gtk::{LoadEvent, SettingsBuilder, WebContext, WebInspectorExt, WebView, WebViewExt};

use crate::app::AppSettings;

use super::mainwindow::MainWindow;

pub struct MainWebView {
    pub webview: WebView,
}

impl MainWebView {
    pub fn new(application: &Application, mainwindow: MainWindow, builder: &Option<Builder>, app_settings: &AppSettings) -> Self {
        let context = WebContext::get_default().unwrap();
        let webview = match builder {
            Some(builder) =>  builder.get_object("webview").unwrap(),
            None => {
                let webview = WebView::with_context(&context);
                let settings = SettingsBuilder::new();
                let settings = if app_settings.enable_dev_tools {
                    settings.enable_developer_extras(true)
                } else { settings };
                let settings = settings.build();
                webview.set_settings(&settings);
                webview
            }
        };
        
        if let Some(on_init) = app_settings.on_app_init {
            on_init(application, &mainwindow.window, &builder, &webview)
        };

        mainwindow.window.add(&webview);        
        webview.connect_context_menu(|_, _, _, _| true );
        
        let weak_webview = webview.clone();

        webview.connect_load_changed(move |_,load_event| 
            if load_event == LoadEvent::Committed {
                let script = 
r"function sendMessageToWebView(command, param) {
    alert(`!!webmesg!!${command}!!${param}`)
}";
                weak_webview.run_javascript(&script, Some(&gio::Cancellable::new()), |_|{});
            }
        );

        let weak_webview = webview.clone();
        let action = gio::SimpleAction::new("devtools", None);
        action.connect_activate(move |_,_| match weak_webview.get_inspector() {
            Some(inspector) => inspector.show(),
            None => println!("Could not show web inspector")
        });
        application.add_action(&action);
        application.set_accels_for_action("app.devtools", &["<CTRL><SHIFT>I"]);

        MainWebView{ 
            webview, 
        }
    }

    pub fn load(&self, uri: &str) {
        self.webview.load_uri(uri);
    }
}
