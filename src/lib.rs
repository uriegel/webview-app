#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod app;
pub mod appsettings;
mod settings;

// TODO GTK: When there is no Glade.main => building window with webview
// TODO GTK: Settings: default widths and height
// TODO GTK: When there is a Glade.main => building window with webview and main.glade, hello_glade example
// TODO GTK: Extending Settings 
// TODO GTK: Mainwindow (HeaderBar -> hello_header example, Subclass MainWindow with new)
// TODO Inject global.js, Messages to and from the webview
