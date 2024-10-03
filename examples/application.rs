use webview_app::application::Application;

fn main() {
    let app = Application::new(
        "de.uriegel.hello", 
            || {
            println!("Bin ativiert")
        });
    app.run();
}