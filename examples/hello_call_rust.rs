use gtk::{Application, ApplicationWindow, Builder, HeaderBar, HeaderBarExt, prelude::BuilderExtManual};
use webkit2gtk::WebView;
use webview_app::{app::App, app::{AppSettings, WarpSettings, connect_msg_callback}};

fn on_init(_: &Application, _: &ApplicationWindow, builder: &Option<Builder>, webview: & WebView) {
    if let Some(builder) = builder {
        let headerbar: HeaderBar = builder.get_object("headerbar").unwrap();
        headerbar.set_subtitle(Some("The subtitle initially set by on_init method"));
        connect_msg_callback(webview, move|cmd: &str, payload: &str|{ 
            if cmd == "subtitle" {
                headerbar.set_subtitle(Some(payload));
            }
        });
    }
}

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            enable_dev_tools: true,
            url: "http://localhost:9999/examples/msgtorust.html".to_string(),
            use_glade: true,
            on_app_init: Some(on_init),
            warp_settings: Some(WarpSettings { 
                port: 9999,
                init_fn: None,
            }),
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}