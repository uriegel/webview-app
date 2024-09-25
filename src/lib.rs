#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod webview;
mod bounds;

// WIndows: initiali_bounds
// TODO Macro std::include_str!
// TODO https://stackoverflow.com/questions/46373028/how-to-release-a-beta-version-of-a-crate-for-limited-public-testing
