//! # webview_app
//! 
//! Integration of a web view in an application window like Electron, 
//! but using rust as programming language. On Windows webview2 will be used as 
//! webview, on Linux it is WebKitGTK 6.
//! Here is an easy example to create and run a simple webview app displaying crates homepage:
//! ``` 
//! use webview_app::{application::Application, webview::WebView};
//! 
//! fn on_activate(app: &Application)->WebView {
//!     WebView::builder(app)
//!         .title("Rust Web View 👍".to_string())
//!         .url("https://crates.io/crates".to_string())
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
//! A tutorial for using weview_app:
//! 
//! https://github.com/uriegel/webview-app

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

// TODO Query string
// TODO Result as return instead of unwraps
// TODO Dark backgroundcolor 
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon default ID, version, crate winresource


