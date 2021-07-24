use gio::{SimpleAction, prelude::ToVariant, traits::ActionMapExt};
use gtk::{HeaderBar, prelude::{BuilderExtManual, HeaderBarExt}};
use webkit2gtk::{traits::WebViewExt};
use webview_app::{app::App, app::{AppSettings, InitData, WarpSettings, connect_msg_callback}};

fn on_init(data: InitData) {
    let initial_state = "".to_variant();
    let webview_clone = data.webview.clone();
    let action = SimpleAction::new_stateful("themes", Some(&initial_state.type_()), &initial_state);
        action.connect_change_state(move |a, s| {
            match s {
            Some(val) => {
                a.set_state(val);
                match val.str() {
                    Some(theme) => 
                        webview_clone.run_javascript(&format!("setTheme('{}')", theme), Some(&gio::Cancellable::new()), |_|{}),
                    None => println!("Could not set theme, could not extract from variant")
                }
            },
            None => println!("Could not set theme")
        }
        });
        data.application.add_action(&action);

    if let Some(builder) = data.builder {
        let headerbar: HeaderBar = builder.object("headerbar").unwrap();
        headerbar.set_subtitle(Some("The subtitle initially set by on_init method"));
        connect_msg_callback(data.webview, move|cmd: &str, payload: &str|{ 
            match cmd {
                "subtitle" => headerbar.set_subtitle(Some(payload)),
                "theme" => action.set_state(&payload.to_variant()),
                _ => {}
            }
        });
    }
}

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
fn main() {
    run_app();
}