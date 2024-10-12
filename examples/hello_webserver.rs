#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.

use serde::{Deserialize, Serialize};
use include_dir::include_dir;
use webview_app::{application::Application, httpserver::HttpServerBuilder, request::{self, request_blocking, Request}, webview::WebView};

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
    let webview = WebView::builder(app)
        .title("Hello Web Server👍".to_string())
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/hello_webserver"))
        .default_contextmenu_disabled()
        .with_http_server(
            HttpServerBuilder::new()
                .port(8888)
                .build()
        )
        .on_http_request(|cmd, json|{
            match cmd {
                "cmd1" => cmd1_http(json),
                "cmd2" => cmd2_http(json),
                _ => "{}".to_string()
                }
        })
        .build();
    
    webview.connect_request(move|request, id, cmd: String, json| {
        match cmd.as_str() {
            "cmd1" => cmd1(request, id, json),
            "cmd2" => cmd2(request, id, json),
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

fn cmd1_http(json: &str)->String {
    let input: Input = request::get_input(&json);
    let res = Output {
        email: "uriegel@hotmail.de".to_string(),
        text: input.text,
        number: input.id + 1,
    };
    request::get_output(&res)
}

fn cmd2(request: &Request, id: String, json: String) {
    request_blocking(request, id, move || {
        let input: Input = request::get_input(&json);

        let outputs: Vec<Output> = 
            (1..100_000)
            .map(|i| Output {
                email: "uriegel@hotmail.de".to_string(),
                text: input.text.clone(),
                number: input.id + i,
            })
            .collect();
        let res = Outputs { outputs };
        request::get_output(&res)
    })
}

fn cmd2_http(json: &str)->String {
    let input: Input = request::get_input(&json);
    let outputs: Vec<Output> = 
        (1..100_000)
        .map(|i| Output {
            email: "uriegel@hotmail.de".to_string(),
            text: input.text.clone(),
            number: input.id + i,
        })
        .collect();
    let res = Outputs { outputs };
    request::get_output(&res)
}

 