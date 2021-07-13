use webview_app::{app::App, app::AppSettings};

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            enable_dev_tools: true,
            url: "http://localhost:9860".to_string(), 
            window_pos_storage_path: Some("hello".to_string()),
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}