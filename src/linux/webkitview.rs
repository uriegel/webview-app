use std::sync::{Arc, Mutex};

use gtk::gio::MemoryInputStream;
use gtk::glib::{Bytes, MainContext};
use gtk::Builder;
use include_dir::Dir;
use webkit6::prelude::*;
use webkit6::{soup::MessageHeaders, Settings, LoadEvent, URISchemeRequest, URISchemeResponse, WebView};

use crate::content_type;
use crate::html;
use crate::javascript::{self, RequestData};
use crate::request::Request;

#[derive(Clone)]
pub struct WebkitView {
    pub webview: WebView,
}

pub struct WebkitViewParams<'a> {
    pub url: &'a str,
    pub debug_url: Option<String>,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub webroot: Option<Arc<Mutex<Dir<'static>>>>,
}

impl WebkitView {
    pub fn new(builder: &Builder, params: WebkitViewParams) -> Self {

        let webview: WebView = builder.object("webview").expect("There must be a child with id 'webview' in the window.ui");
        if params.devtools {
            let settings: Settings = builder.object("webkit_settings").expect("There must be settings by the id of 'webkit_settings' as properties to 'webview' in the window.ui");
            settings.set_enable_developer_extras(true);
        }
        if !params.default_contextmenu {
            webview.connect_context_menu(|_,_,_|true);
        }

        let res = WebkitView {
            webview,
        };

        res.enable_request_scheme();
        match (params.debug_url, params.webroot) {
            (None, Some(webroot)) => {
                res.webview.load_uri("res://webroot/index.html");
                res.enable_resource_scheme(webroot)
            },
            (Some(debug_url), _) => res.webview.load_uri(&debug_url),
            _ => res.webview.load_uri(params.url)
        }

        res.webview.connect_load_changed(move|webview, evt| {
            let webview = webview.clone();
            if evt == LoadEvent::Committed {
                MainContext::default().spawn_local(async move {
                    let script = javascript::get(false, "", false, false);
                    webview.evaluate_javascript_future(&script, None, None).await.expect("error in initial running script");
                });
            }
        });

        res
    }

    pub fn connect_request<F: Fn(&Request, String, String, String) -> bool + 'static>(
        &self,
        on_request: F,
    ) {
        let request = Request {
            webview: self.webview.clone()
        };
        self.webview.connect_script_dialog(move|_, d| {
            let txt = d.message().unwrap();
            let msg = txt.as_str().to_string();
            let request_data = RequestData::new(&msg);
            let cmd = request_data.cmd.to_string();
            let id = request_data.id.to_string();
            let json = request_data.json.to_string();
            on_request(&request, id, cmd, json)
        });
    }

    fn enable_request_scheme(&self) {
        let webview = self.webview.clone();
        self.webview
            .context()
            .expect("Could not get default web context")
            .register_uri_scheme("req", move | req | {
                match req.uri().unwrap().to_string().as_str() {
                    "req://showDevTools" => {
                        if let Some(insp) = webview.inspector() { insp.show(); }
                        WebkitView::send_response(req, 200, "Ok", html::ok());
                    },
                    _ => WebkitView::send_response(req, 404, "Not found", html::not_found())
                }
            });
    }

    fn enable_resource_scheme(&self, webroot: Arc<Mutex<Dir<'static>>>) {
        self.webview
            .context()
            .expect("Could not get default web context")
            .register_uri_scheme("res", move | req | {
                let uri = req.uri().unwrap().to_string();

                let mut file = uri.clone();
                let path = file.split_off(14);

                match webroot
                        .lock()
                        .unwrap()
                        .get_file(path) 
                        .map(|file| file.contents()) {
                    Some(bytes) => {
                        let bs = Bytes::from_static(&bytes);
                        let stream = MemoryInputStream::from_bytes(&bs);
                        req.finish(&stream, bytes.len() as i64, Some(&content_type::get(&uri)));
                    },
                    None => WebkitView::send_response(req, 404, "Not found", html::not_found())
                };
            });
    }

    fn send_response(request: &URISchemeRequest, status: u32, code: &str, body: & 'static str) {
        let bytes = body.as_bytes();
        let bs = Bytes::from_static(bytes);
        let stream = MemoryInputStream::from_bytes(&bs);
        let response = URISchemeResponse::new(&stream, bytes.len() as i64);
        response.set_status(status, Some(code));
        let headers = MessageHeaders::new(webkit6::soup::MessageHeadersType::Response);
        headers.append("Access-Control-Allow-Origin", "*");
        headers.append("Content-Type", "text/html");
        headers.append("Content-Length", &bytes.len().to_string());
        response.set_http_headers(headers);
        request.finish_with_response(&response);                        
    }
}

