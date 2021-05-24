//! # webview_app
//! 
//! Simple integration of a web view in an application window like Electron, 
//! but using rust as programming language. On Windows webview2 will be used as 
//! webview, on Linux it is WebKit GtkWebview2.
//! Here is an easy example to create and run a simple webview app displaying crates homepage:
//! ``` 
//! use webview_app::{app::App, app::AppSettings};
//! 
//! fn run_app() {
//!     let app = App::new(
//!         AppSettings { 
//!             title: "Rust Web View".to_string(),
//!             url: "https://crates.io".to_string(), 
//!             ..Default::default()
//!         }
//!     );
//!     app.run();
//! }
//! 
//! fn main() {
//!     run_app();
//! }
//! ``` 

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod app;
pub mod headers;
mod settings;
mod warp_server;

// TODO Windows: msg to windows
// TODO save Window Pos
