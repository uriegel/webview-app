//! # webview_app
//! 
//! Integration of a web view in an application window like Electron, 
//! but using rust as programming language. On Windows WebView2 will be used as 
//! web view, on Linux it is WebKitGTK.
//! 
//! A simple example to create and run a simple web view app displaying crates homepage:
//! ``` 
//! use webview_app::{application::Application, webview::WebView};
//! 
//! fn on_activate(app: &Application)->WebView {
//!     WebView::builder(app)
//!         .title("Rust Web View 🦞")
//!         .url("https://crates.io/crates")
//!         .default_contextmenu_disabled()
//!         .build();
//! }
//! 
//! fn main() {
//!     Application::new("de.uriegel.hello")
//!     .on_activate(on_activate)
//!     .run();
//! }
//! ``` 
//! A tutorial for using webview_app:
//! 
//! <https://github.com/uriegel/webview-app>

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod application;
pub mod webview;
pub mod request;
mod bounds;
mod params;
mod content_type;
mod javascript;
mod html;

// TODO Result as return instead of unwraps
// TODO Dark backgroundcolor 
// TODO DragDrop
// TODO Doc with examples and panics and global descriptions, module descriptions and links to tutorial
// TODO Keywords like http webview, gtk, webview2 webkit, electron like
// TODO Dependency Gtk4, Webkit, Aswaita for doc


