#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod webview;
mod bounds;
mod callbacks;

// TODO https://stackoverflow.com/questions/46373028/how-to-release-a-beta-version-of-a-crate-for-limited-public-testing
// TODO Windows ICON: build.rs: make resource file with icon 1, version 
// TODO Macro std::include_str!
