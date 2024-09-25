struct Bounds {
    x: Option<i32>,
    y: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
    is_maximized: bool
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