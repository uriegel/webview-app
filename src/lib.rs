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

// TODO Windows new: hello can close

// TODO Windows new: custom_resources
// TODO Windows new: window-titlebar
// TODO Windows new: requests

// TODO Dark backgroundcolor (Windows?)
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon default ID, version, crate winresource


