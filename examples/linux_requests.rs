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
        let request_data = RequestData::new(&msg);
        let cmd = request_data.cmd.to_string();
        let id = request_data.id.to_string();
        let json = request_data.json.to_string();
        let req = Request::new(cmd, id, json);
        on_request2(webview, req);     
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
            let input: Input = Request::get_input(&req.input);
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

pub struct Request {
    cmd: String,
    id: String,
    input: String
}

impl Request {
    pub fn new(cmd: String, id: String, input: String)->Self {
        Self { id, cmd, input }
    }

    pub fn get_input<'a, T>(data: &'a str)->T where T: Deserialize<'a> {
        serde_json::from_str(data).unwrap()
    }

    pub fn get_output<T>(&self, result: &T)->String where T: Serialize {
        serde_json::to_string(result).unwrap()
    }
}

fn test<R: 'static, F: std::future::Future<Output = R> + 'static>(
    on_request: F) {
        spawn_future_local(async move {
            on_request.await
    });
} 

// fn test_blocking<R: 'static, F: std::future::Future<Output = R> + 'static>(
//     on_request: F) {
//         spawn_future_local(async move {
//             let response = gio::spawn_blocking( move|| {
//                 on_request
//     });
// } 

fn on_request2(webview: &webkit6::WebView, req: Request) {
    test(clone!(
        #[weak]
        webview,
        async move {

//             thread::spawn(|| {
//                 let five_seconds = Duration::from_secs(5);
//                 thread::sleep(five_seconds);
//                 // MainContext::default().spawn_local(async move {
//                 //     webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
//                 // });
//             });

            let id = req.id.clone();
            let response = gio::spawn_blocking( move|| {
//                 // let five_seconds = Duration::from_secs(5);
//                 // thread::sleep(five_seconds);

                on_request(&req.cmd, &req)
            })
            .await
            .expect("Task needs to finish successfully.");
            let back: String = format!("result,{},{}", id, response);
//         //    MainContext::default().spawn_local(async move {
            webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
            
        }
    ));
}
