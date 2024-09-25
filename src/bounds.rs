#[derive(Copy, Clone)]
pub struct Bounds {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub is_maximized: bool
}

impl Bounds {
    pub fn save(&self)->() {

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