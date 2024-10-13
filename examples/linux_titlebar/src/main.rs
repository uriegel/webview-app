use gtk::prelude::*;
use gtk::{gio, Window};
use webview_app::application::Application;
use webview_app::webview::WebView;

// fn set_titlebar(app: &adw::Application, webview: &webkit6::WebView)->gtk::Widget {
//     let header_bar = HeaderBar::new();
//     let button = Button::with_label("Dev tools");
//     button.set_action_name(Some("app.devtools"));
//     header_bar.pack_end(&button);

//     let action = ActionEntry::builder("devtools")
//         .activate(clone!(
//             #[weak]
//             webview,
//             move |_, _, _| {
//             if let Some(inspector) = webview.inspector() {
//                 inspector.show();
//             }
//         })).build();
//     app.add_action_entries([action]);
//     app.set_accels_for_action("app.devtools", &["<Shift><Ctrl>I"]);

//     header_bar.upcast::<Widget>()
// }

fn on_activate(app: &Application)->WebView {
    WebView::builder(app)
    .save_bounds()
    .debug_url("https://crates.io/crates/webview_app".to_string())
    .url("https://crates.io/crates".to_string())
    .devtools(true)
    .default_contextmenu_disabled()
    .with_builder("/de/uriegel/webview_app/window.ui".to_string(), |builder| {
        let window: Window = builder.object("window").unwrap();
        window.set_title(Some("Linux Titlebar 👍"));
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
