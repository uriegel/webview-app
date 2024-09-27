#[cfg(target_os = "windows")] 
use std::{env, fs, path::PathBuf};

#[cfg(target_os = "windows")] 
macro_rules! log_warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

#[cfg(target_os = "windows")] 
fn copy(path: &PathBuf, profile: &str, name: &str) {
    let from = path.join("assets").join(name);
    let to = path.join("target").join(profile).join("examples").join(name); 
    //log_warning!("Copy {} {}", from.to_string_lossy(), to.to_string_lossy());
    fs::copy(from, to).unwrap();
}

#[cfg(target_os = "windows")] 
fn copy_windows_dlls() {
    let p = env::current_dir().unwrap();
    let profile = env::var("PROFILE").unwrap();
    copy(&p, &profile, "WebView2Loader.dll");
    copy(&p, &profile, "WebViewApp.dll");        
}


pub fn main() {
    #[cfg(target_os = "windows")]
    copy_windows_dlls();
}