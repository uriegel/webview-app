#[cfg(target_os = "linux")]
use webview_app::app::App;

use webview_app::test;

#[cfg(target_os = "linux")]
fn run_app() {
    let app = App::new("test.uriegel.de");
    app.run();
}

#[cfg(target_os = "windows")]
fn run_app() {
    println!("Please wait, not implemented...");
}

fn main() {
    test();
    run_app();
}