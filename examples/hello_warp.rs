use webview_app::{app::App, app::AppSettings};

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            warp_port: 9999,
            enable_dev_tools: true,
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}