use std::sync::{Arc, Mutex};
use std::time::Duration;

use gtk::gio::MemoryInputStream;
use gtk::glib::{self, clone, spawn_future_local, timeout_future_with_priority, Bytes, MainContext, Priority};
use gtk::Builder;
use include_dir::Dir;
use webkit6::prelude::*;
use webkit6::{soup::MessageHeaders, LoadEvent, URISchemeRequest, URISchemeResponse, WebView};
use async_channel::Sender;

use crate::content_type;
use crate::html;
use crate::javascript::{self, RequestData};
use crate::request::Request;

#[derive(Clone)]
pub struct WebkitView {
    pub webview: WebView,
    event_sender: Sender<String>
}

#[derive(Clone)]
pub struct WebViewHandle {
    event_sender: Sender<String>    
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
        let (sender, receiver) = async_channel::unbounded::<String>();

        let webview: WebView = builder.object("webview").expect("There must be a child with id 'webview' in the window.ui");
        webview.set_visible(false);
        if params.devtools {
            let settings = webkit6::prelude::WebViewExt::settings(&webview);
            settings.unwrap().set_enable_developer_extras(true);
        }
        if !params.default_contextmenu {
            webview.connect_context_menu(|_,_,_|true);
        }

        glib::spawn_future_local(clone!(
            #[weak] webview, async move {
                while let Ok(script) = receiver.recv().await {
                    webview.evaluate_javascript_future(&script, None, None).await.unwrap();
                }
            }
        ));

        let res = WebkitView {
            webview,
            event_sender: sender
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
            if evt == LoadEvent::Committed {
                MainContext::default().spawn_local(clone!(
                    #[weak]
                    webview,
                    async move {
                    let script = javascript::get(false, "", false, false);
                    webview.evaluate_javascript_future(&script, None, None).await.expect("error in initial running script");
                }));
                spawn_future_local(clone!(
                    #[weak]
                    webview,
                    async move {
                        timeout_future_with_priority(Priority::DEFAULT, Duration::from_millis(20)).await;
                        webview.set_visible(true);
                    }
                ));                
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
                let mut path = file.split_off(14);
                let path = if path.starts_with("webroot/") {
                    path.split_off(8)
                } else {
                    path
                };

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

    pub fn get_handle(&self)->WebViewHandle {
        WebViewHandle { 
            event_sender: self.event_sender.clone()
        }
    }

    pub fn start_evaluate_script(handle: crate::webview::WebViewHandle, script: &str) {  
        handle.handle.event_sender.send_blocking(script.to_string()).unwrap();
    }
}

