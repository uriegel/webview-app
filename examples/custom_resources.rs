#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use include_dir::include_dir;
use serde::{Deserialize, Serialize};
use webview_app::{application::Application, webview::WebView};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub text: String,
    pub id: i32
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub text: String,
    pub email: String,
    pub number: i32
}

fn on_activate(app: &Application)->WebView {
    let webview = WebView::builder(app)
        .title("Website form custom resources 👍".to_string())
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/custom_resources"))
        .default_contextmenu_disabled()
        .build();

    webview.on_request(|cmd, req|{
        match cmd {
            "cmd1" => {
                let input: Input = req.get_input();
                let res = Output {
                    email: "uriegel@hotmail.de".to_string(),
                    text: input.text,
                    number: input.id + 1,
                };
                req.get_output(&res)
            },
            "cmd2" => {
                let res = Output {
                    email: "uriegel@hotmail.de".to_string(),
                    text: "Return fom cmd2".to_string(),
                    number: 456,
                };
                req.get_output(&res)
            },
            _ => {
                let res = Output {
                    email: "uriegel@hotmail.de".to_string(),
                    text: "Unknown request".to_string(),
                    number: 0,
                };
                req.get_output(&res)
            }
        }
    });

    webview
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}


