use gio::{SimpleAction, prelude::ToVariant, traits::ActionMapExt};
use gtk::{Application, ApplicationWindow, Builder, HeaderBar, prelude::{BuilderExtManual, HeaderBarExt}};
use webkit2gtk::{WebView, traits::WebViewExt};
use webview_app::{app::App, app::{AppSettings, WarpSettings, connect_msg_callback}};

fn on_init(application: &Application, _: &ApplicationWindow, builder: &Option<Builder>, webview: &WebView) {
    let initial_state = "".to_variant();
    let weak_webview = webview.clone();
    let action = SimpleAction::new_stateful("themes", Some(&initial_state.type_()), &initial_state);
        action.connect_change_state(move |a, s| {
            match s {
            Some(val) => {
                a.set_state(val);
                match val.str() {
                    Some(theme) => 
                        weak_webview.run_javascript(&format!("setTheme('{}')", theme), Some(&gio::Cancellable::new()), |_|{}),
                    None => println!("Could not set theme, could not extract from variant")
                }
            },
            None => println!("Could not set theme")
        }
        });
        application.add_action(&action);

    if let Some(builder) = builder {
        let headerbar: HeaderBar = builder.object("headerbar").unwrap();
        headerbar.set_subtitle(Some("The subtitle initially set by on_init method"));
        connect_msg_callback(webview, move|cmd: &str, payload: &str|{ 
            match cmd {
                "subtitle" => headerbar.set_subtitle(Some(payload)),
                "theme" => action.set_state(&payload.to_variant()),
                _ => {}
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