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
    pub fn save(&self)->() {
        let json = serde_json::to_string(&self).unwrap();
    }

    pub fn restore()->Self {
        Bounds {
            x: None,
            y: None,
            width: None,
            height: None,
            is_maximized: false
        }
    }
}