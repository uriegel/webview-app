use gtk::prelude::*;

pub struct Application<'a> {
    appid: &'a str,
    app: adw::Application
}

impl<'a> Application<'a> {
    pub fn new(appid: &'a str, on_activate: impl Fn() + 'static)->Self {
        let app = adw::Application::builder()
            .application_id(appid)
            .build();

        app.connect_activate(move |_app| {
            on_activate();
        });
        Self {
            appid,
            app
        }
    }

    pub fn run(&self)->u32 {
        self.app
            .run()
            .value() as  u32
    }
}
