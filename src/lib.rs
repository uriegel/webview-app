#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod application;
pub mod webview;
pub mod request;
pub mod httpserver;
mod bounds;
mod params;
mod content_type;
mod javascript;
mod html;
mod threadpool;

// TODO http requests
// TODO CORS (react)
// TODO Windows res and req scheme
// TODO fs_extra::copy_items_with_progress
// TODO Trash: https://docs.rs/trash/latest/trash/
// TODO Windows Headerbar 
// TODO Dark backgroundcolor
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon 1, version 
// TODO Simple webserver with range for videos

