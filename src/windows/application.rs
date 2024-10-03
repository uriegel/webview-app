use super::raw_funcs::load_raw_funcs;

#[derive(Clone)]
pub struct Application {
    pub appid: String
}

impl Application {
    pub fn new(appid: &str)->Self {
        Self {
            appid: appid.to_string()
        }
    }
    
    pub fn get_appid(&self)->String {
        self.appid.clone()
    }

    pub fn on_activate(&self, val: impl Fn() + 'static) {
        val();
    }

    pub fn run(&self)->u32 {
        (load_raw_funcs("").run)()
    }
}
