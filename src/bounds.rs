use std::{fs::{self}, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub is_maximized: bool
}

impl Bounds {
    pub fn restore(config_dir: &str)->Option<Self> {
        fs::read_to_string(
            Path::new(config_dir).join("bounds.json"))
                .ok()
                .and_then(|json| serde_json::from_str(&json).ok())
    }

    pub fn save(&self, config_dir: &str)->() {
        let json = serde_json::to_string(&self).unwrap();
        fs::write(Path::new(config_dir).join("bounds.json"), json).unwrap();
    }
}