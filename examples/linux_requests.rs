#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use gtk::{gio, glib::{self, clone, spawn_future_local, MainContext}};
use include_dir::include_dir;
use webview_app::{application::Application, webview::WebView};
use webkit6::prelude::*;

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
        .title("Linux requests 👍".to_string())
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/custom_resources"))
        .default_contextmenu_disabled()
        .build();
    
    webview.get_webkit().connect_script_dialog(move|webview, d| {
        let txt = d.message().unwrap();
        let msg = txt.as_str().to_string();
        spawn_future_local(clone!(
            #[weak]
            webview,
            async move {
                let msg = txt.as_str().to_string();
                let request_data = RequestData::new(&msg);
                let cmd = request_data.cmd.to_string();
                let id = request_data.id.to_string();
                let json = request_data.json.to_string();



                let response = gio::spawn_blocking( move|| {
                    let five_seconds = Duration::from_secs(5);
                    thread::sleep(five_seconds);

                    on_request(&cmd, &Request::new(&json))
                })
                .await
                .expect("Task needs to finish successfully.");
                let back: String = format!("result,{},{}", id, response);
                MainContext::default().spawn_local(async move {
                    webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
                });
            }
        ));
        println!("Jetzt zuende");
        true
    });
    webview
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}

fn on_request(cmd: &str, req: &Request)->String {
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
}

pub struct RequestData<'a> {
    pub cmd: &'a str,
    pub id: &'a str,
    pub json: &'a str
}

impl <'a>RequestData<'a> {
    pub fn new(msg: &'a str)->RequestData<'a> {
        let msg = &msg[8..];
        let idx = msg.find(',').unwrap();
        let 
        cmd = &msg[..idx];
        let msg= &msg[idx+1..];
        let idx = msg.find(',').unwrap();
        let id = &msg[..idx];
        let json = &msg[idx+1..];
        let _ = &json[1..2];
        RequestData {
            cmd,
            id,
            json
        }
    }
}

pub struct Request<'a> {
    input: &'a str
}

impl<'a> Request<'a> {
    pub fn new(input: &'a str)->Self {
        Self { input }
    }

    pub fn get_input<T>(&self)->T where T: Deserialize<'a> {
        serde_json::from_str(&self.input).unwrap()
    }

    pub fn get_output<T>(&self, result: &T)->String where T: Serialize {
        serde_json::to_string(result).unwrap()
    }
}

