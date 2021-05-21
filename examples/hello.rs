use webview_app::{app::App, appsettings::AppSettings};

#[cfg(target_os = "linux")]
fn run_app() {
    let app = App::new(
        AppSettings{
            title: "Rust Web View".to_string(),
            ..Default::default()
        });
    app.run();
}

#[cfg(target_os = "windows")]
fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View".to_string(),
            url: "https://google.de".to_string(), 
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}