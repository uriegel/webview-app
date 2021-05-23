use warp::{Filter, filters::BoxedFilter, reply::{Json, json}};
use webview_app::{app::App, app::AppSettings};
use serde::{Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WarpItem {
    pub name: String,
    pub display: String,
    pub capacity: u64,
}

async fn get_items()->Result<Json, warp::Rejection> {
    println!("Request called");
    let item = WarpItem { 
        capacity:123, 
        display: "Warp returning jon data".to_string(), 
        name: "warp filter".to_string()
    };
    Ok(json(&item))
    //Err(warp::reject())
}

fn get_filters()->BoxedFilter<(Json,)>  {
    warp::get()
    .and(warp::path("requests"))
    .and(warp::path("getitems"))
    .and(warp::path::end())
    .and_then(get_items).boxed()
}

fn run_app() {
    let app = App::new(
        AppSettings { 
            title: "Rust Web View 👍".to_string(),
            warp_port: 9999,
            warp_json_filters: Some(get_filters),
            url: "http://localhost:9999/examples/warpfilters.html".to_string(),
            enable_dev_tools: true,
            ..Default::default()
        }
    );
    app.run();
}

fn main() {
    run_app();
}