//#![windows_subsystem = "windows"]

use webview_app::{app::App, app::AppSettings};

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            url: "https://test/crates.io".to_string(), 
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}