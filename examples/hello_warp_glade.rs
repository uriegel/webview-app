use std::{any::Any, thread};

use gio::{glib::{MainContext, PRIORITY_DEFAULT, Receiver, Sender}, prelude::Continue};
use gtk::prelude::GtkWindowExt;
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use warp::fs::dir;
use webview_app::{app::App, app::AppSettings, app::{AppState, InitData, WarpInitData, WarpSettings}, headers::add_headers};
use warp::{Filter, reply::{Json, json}};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WarpItem {
    pub name: String,
    pub display: String,
    pub capacity: u64,
}

#[derive(Deserialize)]
struct PostItem {
    path: String,
}

async fn get_item()->Result<Json, warp::Rejection> {
    let item = WarpItem { 
        capacity:123, 
        display: "Warp returning json data".to_string(), 
        name: "warp filter".to_string()
    };
    Ok(json(&item))
    //Err(warp::reject())
}

async fn post_item(param: PostItem, state: AppState)->Result<Json, warp::Rejection> {
    let item = WarpItem { 
        capacity:123, 
        display: "Warp returning json data".to_string(), 
        name: param.path.clone()
    };


    let s = state.lock().unwrap();
    let r: &dyn Any = s.as_ref();
    if let Some(dc) = r.downcast_ref::<SuperData>() {
        println!("Thread ID in warp callback {:?}", thread::current().id());
        dc.sender.send(true).ok();
    }


    // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object


    Ok(json(&item))
    //Err(warp::reject())
}

struct SuperData {
    pub sender: Sender<bool>,
}

fn on_init(data: InitData) {
    println!("Thread ID in app init {:?}", thread::current().id());
    let (sender, receiver): (Sender<bool>, Receiver<bool>) = MainContext::channel::<bool>(PRIORITY_DEFAULT);
    let mut val = data.state.lock().unwrap();
    *val = Box::new(SuperData{ sender });

    let win_clone = data.window.clone();

    receiver.attach( None, move |val | {
        println!("Thread ID in receiver {:?}, val: {}", thread::current().id(), val);
        win_clone.maximize();
        Continue(true)
    });        
}

fn server(rt: &Runtime, data: WarpInitData) {
    let state = data.state.clone();
    rt.spawn(async move {

        let get_json = 
            warp::get()
            .and(warp::path("requests"))
            .and(warp::path("getitem"))
            .and(warp::path::end())
            .and_then(get_item);

        let post_json = 
            warp::post()
            .and(warp::path("requests"))
            .and(warp::path("postitem"))
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(move |p|{post_item(p, state.clone())});

        let route_static = dir(data.static_dir)
            .map(add_headers);

        let routes = 
            get_json
            .or(post_json)
            .or(route_static);

        warp::serve(routes)
            .run(data.socket_addr)
            .await;        
    });
}

#[cfg(target_os = "linux")]
fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            url: "http://localhost:9999/examples/warpfilters.html".to_string(),
            use_glade: true,
            warp_settings: Some(WarpSettings { 
                port: 9999,
                init_fn: Some(server),
            }),
            on_app_init: Some(on_init),
            enable_dev_tools: true,
            ..Default::default()
        }
    );
    app.run();
}

#[cfg(target_os = "linux")]
fn main() {
    run_app();
}
