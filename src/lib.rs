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

// TODO Windows Check alternating img from res request
// TODO Windows Devtools WebviewRequest
// TODO Webview JsonRequests
// TODO react test site debug: JsonRequests CORS?
// TODO Windows Headerbar 
// TODO DragDrop
// TODO Windows ICON: build.rs: make resource file with icon 1, version 
// TODO Simple webserver with range for videos

