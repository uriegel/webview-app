use webview_app::{app::App, app::AppSettings};

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            webroot: "examples".to_string(),
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}