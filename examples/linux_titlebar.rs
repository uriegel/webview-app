use adw::HeaderBar;
use gio::ActionEntry;
use glib::clone;
use gtk::prelude::*;
use gtk::Button;
use gtk::Widget;
use webview_app::application::Application;
use webview_app::webview::WebView;
use webkit6::*;
use webkit6::prelude::*;

fn set_titlebar(app: &adw::Application, webview: &webkit6::WebView)->gtk::Widget {
    let header_bar = HeaderBar::new();
    let button = Button::with_label("Dev tools");
    button.set_action_name(Some("app.devtools"));
    header_bar.pack_end(&button);

    let action = ActionEntry::builder("devtools")
        .activate(clone!(
            #[weak]
            webview,
            move |_, _, _| {
            if let Some(inspector) = webview.inspector() {
                inspector.show();
            }
        })).build();
    app.add_action_entries([action]);
    app.set_accels_for_action("app.devtools", &["<Shift><Ctrl>I"]);

    header_bar.upcast::<Widget>()
}

fn on_activate(app: &Application)->WebView {
    WebView::builder(app)
    .title("Linux Titlebar 👍".to_string())
    .titlebar(set_titlebar)
    .save_bounds()
    .debug_url("https://crates.io/crates/webview_app".to_string())
    .url("https://crates.io/crates".to_string())
    .devtools(true)
    .default_contextmenu_disabled()
    .build()
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}
