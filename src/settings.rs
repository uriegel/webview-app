use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::{ErrorKind, Read, Write}, path::PathBuf};

const SIZE_NAME: &str = "windowSize";

#[derive(Serialize, Deserialize)]
pub struct SizeSettings {
    pub width: i32,
    pub height: i32,
}

pub fn initialize_size(width: i32, height: i32) -> (i32, i32) {
    match initialize(SIZE_NAME) {
        Some(contents) => {
            let settings: SizeSettings = serde_json::from_str(&contents).unwrap();
            (settings.width, settings.height)
        }            
        None => (width, height)
    }
}
            
pub fn save_size(size: (i32, i32)) {
    let settings = SizeSettings {width: size.0, height: size.1};
    let json = serde_json::to_string(&settings).unwrap();
    save_settings(SIZE_NAME, &json);
}

fn save_settings(name: &str, content: &str) {
    let settings_path = get_settings_path(name);
    let mut file = File::create(settings_path).unwrap();
    file.write(content.as_bytes()).expect("Unable to write settings");
}

fn initialize(name: &str) -> Option<String> {
    let settings = get_settings_path(name);
    let result = match File::open(settings) {
        Ok(mut file) => {   
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Some(contents)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let settings = get_settings_path(name);
            let mut path = settings.clone();
            path.pop();
            if fs::metadata(path.clone()).is_ok() == false {
                fs::create_dir(path).unwrap();
            }
            None
        },
        Err(e) => panic!("Error: {:?}", e)
    };
    result
}

fn get_settings_path(name: &str) -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    [ 
        home_dir.to_str().unwrap(), 
        ".config", 
        "commander",
        name].iter().collect()
}

