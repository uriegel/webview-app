#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod webview;
mod bounds;
mod params;
mod content_type;

// TODO Windows ICON: build.rs: make resource file with icon 1, version 

