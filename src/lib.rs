#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod webview;
mod bounds;
mod callbacks;

// TODO In VisualStudio sln, make a script to copy app.dll and loader dll (debug AND release) to an assets directory
// in TOML:
// [package]
// # ...
//
// # Include additional files when packaging the crate
// include = ["assets/*"]
//
// in src code:
// pub const CONFIG_BYTES: &'static [u8] = include_bytes!("assets/debug/apploader.dll");

// TODO Windows ICON: build.rs: make resource file with icon 1, version 
// TODO Macro std::include_str!
