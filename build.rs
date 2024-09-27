use std::{env, fs, path::PathBuf};

macro_rules! log_warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn copy(path: &PathBuf, profile: &str, name: &str) {
    let from = path.join("assets").join(name);
    let to = path.join("target").join(profile).join("examples").join(name); 
    log_warning!("Copy {} {}", from.to_string_lossy(), to.to_string_lossy());
    fs::copy(from, to).unwrap();
}

pub fn main() {
    let p = env::current_dir().unwrap();
    let profile = env::var("PROFILE").unwrap();
    copy(&p, &profile, "WebView2Loader.dll");
    copy(&p, &profile, "WebViewApp.dll");        
}