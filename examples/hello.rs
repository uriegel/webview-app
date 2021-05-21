use webview_app::app::App;

#[cfg(target_os = "linux")]
fn run_app() {
    let app = App::new("test.uriegel.de");
    app.run();
}

#[cfg(target_os = "windows")]
fn run_app() {
    let app = App::new();
    app.run();
}

fn main() {
    run_app();
}