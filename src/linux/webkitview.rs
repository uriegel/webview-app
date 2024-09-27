use adw::Application;
// use gtk::gio::MemoryInputStream;
// use gtk::glib::Bytes;
use webkit6::prelude::*;
use webkit6::WebView;

use super::mainwindow::MainWindow;

pub struct WebkitView {
    pub _webview: WebView
}

pub struct WebkitViewParams<'a> {
    pub _application: &'a Application, 
    pub mainwindow: MainWindow, 
    pub url: &'a str,
    pub devtools: bool,
    pub default_contextmenu: bool
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

        webview.load_uri(params.url);

        WebkitView {
            _webview: webview
        }
    }

//     fn enable_resource_scheme(&self) {
//         self.webview
//             .context()
//             .expect("Could not get default web context")
//             .register_uri_scheme("res", | req | {
//                 let uri = req.uri().unwrap().to_string();

//                 let test_result = 
// r##"<!DOCTYPE html>

// <html lang="de" >
// <head>
//     <title>Test</title>
//     <link rel="stylesheet" href="css/styles.css">
//     <meta charset="utf-8">
// </head>

// <body>
//     <h1>Test Web Page</h1>

//     <p>
//         <button id="button">Test 1</button>
//         <button id="button2">Test 2</button>
//         <button id="button3">Test 3</button>
//         <button id="buttonDevTools">Dev Tools</button>
//     </p>
//     <p>
//         <img src="images/image.jpg"/>
//     </p>
//     <p>
//         <img src="http://localhost:2222/get/image?path=forest.jpg" />
//     </p>
//     <div id="dragzone">Drag files<br>but only in without Debugger started app</div>
//     <script src="scripts/script.js"></script>

// </body>
// </html>"##;
//                 let bytes = test_result.as_bytes();
//                 let bs = Bytes::from_static(bytes);

//                 let stream = MemoryInputStream::from_bytes(&bs);
//                 req.finish(&stream, bytes.len() as i64, Some(&content_type::get(&uri)));
//             });
//     }
}