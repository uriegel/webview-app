#[cfg(target_os = "linux")]
pub mod app;
#[cfg(target_os = "linux")]
mod mainwindow;
#[cfg(target_os = "linux")]
mod settings;

pub fn test() {
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    println!("Very early hello example test");
}

// TODO GTK: When there is no Glade.main => building window with webview
// TODO GTK: Settings: default widths and height
// TODO GTK: When there is a Glade.main => building window with webview and main.glade, hello_glade example
// TODO GTK: Extending Settings 
// TODO GTK: Mainwindow (HeaderBar -> hello_header example, Subclass MainWindow with new)
// TODO GTK: Properties like window title
// TODO Linux windows dependant sources
// TODO Windoes Window app
// TODO add webviews