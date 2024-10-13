use gtk::prelude::*;
use gtk::{gio, Button};
use webkit6::prelude::*;
use webview_app::application::Application;
use webview_app::webview::WebView;

fn on_activate(app: &Application)->WebView {
    WebView::builder(app)
    .save_bounds()
    .debug_url("https://crates.io/crates/webview_app".to_string())
    .url("https://crates.io/crates".to_string())
    .devtools(true)
    .default_contextmenu_disabled()
    .with_builder("/de/uriegel/webview_app/window.ui".to_string(), |builder| {
        let webview: webkit6::WebView = builder.object("webview").unwrap();
        let button: Button = builder.object("button").unwrap();
        button.connect_clicked(move|_| { 
            webview.inspector().inspect(|inspector|inspector.show());
        });
    })
    .build()
}

fn main() {

    gio::resources_register_include!("linux_titlebar.gresource")
    .expect("Failed to register resources.");

    Application::new("de.uriegel.hello")
        .on_activate(on_activate)
        .run();
}
