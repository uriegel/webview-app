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

// TODO fs_extra::copy_items_with_progress
// TODO Trash: https://docs.rs/trash/latest/trash/
// TODO Dark backgroundcolor (Windows?)
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon default ID, version, crate winresource


