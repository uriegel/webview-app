#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use std::{cell::RefCell, thread, time::Duration};

use serde::{Deserialize, Serialize};
use include_dir::include_dir;
use webview_app::{application::Application, request::{self, request_blocking, Request}, webview::WebView};

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outputs {
    pub outputs: Vec<Output>
}

fn on_activate(app: &Application)->WebView {
    let event_index = RefCell::new(0);
    let webview = WebView::builder(app)
        .title("Requests 👍".to_string())
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/custom_resources"))
        .default_contextmenu_disabled()
        .build();
    
    let webview_clone = webview.clone();
    webview.connect_request(move|request, id, cmd: String, json| {
        match cmd.as_str() {
            "cmd1" => cmd1(request, id, json),
            "cmd2" => cmd2(request, id),
            "cmdE" => {
                *event_index.borrow_mut() += 1;
                webview_clone.eval(&format!("onEvent({})", event_index.borrow()));
            },
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

fn cmd1(request: &Request, id: String, json: String) {
    request_blocking(request, id, move || {
        let input: Input = request::get_input(&json);
        let res = Output {
            email: "uriegel@hotmail.de".to_string(),
            text: input.text,
            number: input.id + 1,
        };
        request::get_output(&res)
    })
}

fn cmd2(request: &Request, id: String) {
    request_blocking(request, id, move || {
        let five_seconds = Duration::from_secs(5);
        thread::sleep(five_seconds);

        let res = Output {
            email: "uriegel@hotmail.de".to_string(),
            text: "Return fom cmd2  sd fd fdsf dsfdsg fdg dfg dfgdfgfdgdfgdfgdffdg dfg dfg dfgdfg dfg dfgdfg dfg dfg".to_string(),
            number: 222,
        };
        request::get_output(&res)
    })
}

 