use std::cell::RefCell;
use std::rc::Rc;

use adw::Application;
use gtk::gio::MemoryInputStream;
use gtk::glib::Bytes;
use include_dir::Dir;
use webkit6::prelude::*;
use webkit6::URISchemeResponse;
use webkit6::WebView;

use crate::content_type;

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

        match (params.debug_url, params.webroot) {
            (None, Some(webroot)) => {
                res.webview.load_uri("res://webroot/index.html");
                res.enable_resource_scheme(webroot)
            },
            (Some(debug_url), _) => res.webview.load_uri(&debug_url),
            _ => res.webview.load_uri(params.url)
        }
        res
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
                    None => {
                        let result404 = 
r##"<!DOCTYPE html>
<html>
<head>
    <title>Not Found</title>
    <meta charset="utf-8">
</head>
<body>
    <h1>Not Found</h1>
                    
    <p>
        Sorry, I cannot find what you're looking for
    </p>
</body>
</html>"##;
                        let bytes = result404.as_bytes();
                        let bs = Bytes::from_static(bytes);
                        let stream = MemoryInputStream::from_bytes(&bs);
                        let response = URISchemeResponse::new(&stream, bytes.len() as i64);
                        response.set_status(404, Some("Not Found"));
                        req.finish_with_response(&response);                        
                    }
                };
            });
    }
}