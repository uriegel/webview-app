use webview_app::app::App;

#[cfg(target_os = "linux")]
use webview_app::test;

fn main() {
    test();

    let app = App::new("test.uriegel.de");
    app.run();
}