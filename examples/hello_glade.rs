use webview_app::{app::App, app::AppSettings};

#[cfg(target_os = "linux")]
fn run_app() {
    let app = App::new(
        AppSettings{
            title: "Rust Web View".to_string(),
            url: "https://crates.io".to_string(), 
            use_glade: true,
            ..Default::default()
        });
    app.run();
}

#[cfg(target_os = "linux")]
fn main() {
    run_app();
}