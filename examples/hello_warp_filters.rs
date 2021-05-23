use std::net::SocketAddr;

use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use warp::fs::dir;
use webview_app::{app::App, app::{AppSettings, WarpSettings}, warp_server::add_headers};
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

async fn post_item(param: PostItem)->Result<Json, warp::Rejection> {
    let item = WarpItem { 
        capacity:123, 
        display: "Warp returning json data".to_string(), 
        name: param.path.clone()
    };
    Ok(json(&item))
    //Err(warp::reject())
}

fn server(rt: &Runtime, socket_addr: SocketAddr, static_dir: String) {
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
            .and_then(post_item);

        let route_static = dir(static_dir)
            .map(add_headers);

        let routes = 
            get_json
            .or(route_static)
            .or(post_json);

        warp::serve(routes)
            .run(socket_addr)
            .await;        
    });
}

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            url: "http://localhost:9999/examples/warpfilters.html".to_string(),
            warp_settings: Some(WarpSettings { 
                port: 9999,
                init_fn: Some(server),
            }),
            enable_dev_tools: true,
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}
