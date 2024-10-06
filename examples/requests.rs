#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use include_dir::include_dir;
use webview_app::{application::Application, request::{self, request_blocking}, webview::WebView};

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
        .title("Requests 👍".to_string())
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/custom_resources"))
        .default_contextmenu_disabled()
        .build();
    
    webview.connect_request(|webview: &WebView, id, cmd: String, json| {
        match cmd.as_str() {
            "cmd1" => cmd1(webview, id, json),
            "cmd2" => cmd2(webview, id),
            _ => {}
        }
        true
    });
    webview
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}

fn cmd1(webview: &WebView, id: String, json: String) {
    request_blocking(webview.clone(), id, move || {
        let input: Input = request::get_input(&json);
        let res = Output {
            email: "uriegel@hotmail.de".to_string(),
            text: input.text,
            number: input.id + 1,
        };
        request::get_output(&res)
    })
}

fn cmd2(webview: &WebView, id: String) {
    request_blocking(webview.clone(), id, move || {
    let five_seconds = Duration::from_secs(5);
    thread::sleep(five_seconds);
    let res = Output {
            email: "uriegel@hotmail.de".to_string(),
            text: "Return fom cmd2".to_string(),
            number: 456,
        };
        request::get_output(&res)
    })
}

 