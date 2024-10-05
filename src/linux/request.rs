use gtk::glib::MainContext;
use webkit6::prelude::*;

use serde::{Deserialize, Serialize};
use webkit6::WebView;

pub struct Request<'a> {
    id: &'a str,
    input: &'a str,
    webview: WebView
}

impl<'a> Request<'a> {
    pub fn new(webview: WebView, id: &'a str, input: &'a str)->Self {
        Self { webview, id, input }
    }

    pub fn get_input<T>(&self)->T where T: Deserialize<'a> {
        serde_json::from_str(self.input).unwrap()
    }

    pub fn send_result<T>(&self, result: &T) where T: Serialize {
        let res = serde_json::to_string(result).unwrap();
        let back: String = format!("result,{},{}", self.id, res);
        let webview = self.webview.clone();
        MainContext::default().spawn_local(async move {
            webview.evaluate_javascript_future(&format!("WebView.backtothefuture('{}')", back), None, None).await.expect("error in initial running script");
            //self.webview.evaluate_javascript(&format!("WebView.backtothefuture('{}')", back), None, None, None::<&_>, |_|{});
        });        
    }
}