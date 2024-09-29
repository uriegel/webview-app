#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod webview;
mod bounds;
mod params;
mod content_type;
mod javascript;
mod html;

// TODO Windows Devtools WebviewRequest
// TODO Windows Headerbar 
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon 1, version 

