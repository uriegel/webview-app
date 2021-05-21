pub struct AppSettings {
    #[cfg(target_os = "linux")]
    pub application_id: String,
    pub title: String,
    pub width: i16,
    pub height: i16,
    pub save_window_pos: bool,
    pub url: String,
}

#[cfg(target_os = "linux")]
impl Default for AppSettings {
    fn default()->Self { 
        Self {
            width: 800,
            height: 600,
            save_window_pos: true,
            title: "".to_string(),
            url: "".to_string()
        }   
    }
}

#[cfg(target_os = "windows")]
impl Default for AppSettings {
    fn default()->Self { 
        Self {
            width: 800,
            height: 600,
            save_window_pos: true,
            title: "".to_string(),
            url: "".to_string()
        }   
    }
}