use webview_app::application::Application;

fn main() {
    Application::new("de.uriegel.hello")
        .on_activate(|| {
            println!("Bin ativiert")
        })
        .run();
}