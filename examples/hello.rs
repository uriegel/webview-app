use webview_app::{app::App, app::AppSettings};

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            url: "https://crates.io".to_string(), 
            window_pos_storage_path: Some("hello".to_string()),
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}