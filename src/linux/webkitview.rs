use std::cell::RefCell;
use std::rc::Rc;

use adw::Application;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use gtk::glib::MainContext;
use include_dir::Dir;
use webkit6::prelude::*;
use webkit6::soup::MessageHeaders;
use webkit6::LoadEvent;
use webkit6::URISchemeRequest;
use webkit6::URISchemeResponse;
use webkit6::WebView;

use crate::content_type;
use crate::html;
use crate::javascript;

use super::mainwindow::MainWindow;

pub struct WebkitView {
    pub webview: WebView
}

pub struct WebkitViewParams<'a> {
    pub _application: &'a Application, 
    pub mainwindow: MainWindow, 
    pub url: &'a str,
    pub debug_url: Option<String>,
    pub devtools: bool,
    pub default_contextmenu: bool,
    pub webroot: Option<Rc<RefCell<Dir<'static>>>>
}

impl WebkitView {
    pub fn new(params: WebkitViewParams) -> Self {
        let webview = WebView::builder()
            .build();
        params.mainwindow.window.set_child(Some(&webview));
        if params.devtools {
            let settings = webkit6::prelude::WebViewExt::settings(&webview);
            settings.unwrap().set_enable_developer_extras(true);
        }
        if !params.default_contextmenu {
            webview.connect_context_menu(|_,_,_|true);
        }

        let res = WebkitView {
            webview
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

        res.webview.connect_load_changed(|webview, evt| {
            let webview = webview.clone();
            if evt == LoadEvent::Committed {
                MainContext::default().spawn_local(async move {
                    // TODO 
                    let script = javascript::get(false, "Das ist der Titel", 0, false);
                    webview.evaluate_javascript_future(&script, None, None).await.expect("error in initial running script");
                });
            }
        });

        res
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

    fn enable_resource_scheme(&self, webroot: Rc<RefCell<Dir<'static>>>) {
        self.webview
            .context()
            .expect("Could not get default web context")
            .register_uri_scheme("res", move | req | {
                let uri = req.uri().unwrap().to_string();

                let mut file = uri.clone();
                let path = file.split_off(14);

                match webroot
                        .borrow()
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