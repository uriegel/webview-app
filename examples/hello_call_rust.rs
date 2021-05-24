use webview_app::{app::App, app::{AppSettings, WarpSettings}};

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            enable_dev_tools: true,
            url: "http://localhost:9999/examples/msgtorust.html".to_string(),
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