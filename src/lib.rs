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

// TODO Linux send back to js a very large json object (and measure time)
// TODO Windows send back to js a very large json object (and measure time)
// TODO runtime for windows and linux
// TODO Windows res and req scheme
// TODO Windows Headerbar 
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon 1, version 
// TODO Simple webserver with range for videos

