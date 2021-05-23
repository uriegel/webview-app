use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use chrono::Utc;
use tokio::runtime::Runtime;
use warp::{Filter, Reply, fs::{File, dir}, http::HeaderValue, hyper::{Body, HeaderMap, Response}};

use crate::app::WarpSettings;

fn create_headers() -> HeaderMap {
    let mut header_map = HeaderMap::new();
    let now = Utc::now();
    let now_str = now.format("%a, %d %h %Y %T GMT").to_string();
    header_map.insert("Expires", HeaderValue::from_str(now_str.as_str()).unwrap());
    header_map.insert("Server", HeaderValue::from_str("webview-app").unwrap());
    header_map
}

pub fn add_headers(reply: File)->Response<Body> {
    let mut res = reply.into_response();
    let headers = res.headers_mut();
    let header_map = create_headers();
    headers.extend(header_map);
    res
}

pub fn start(rt: &Runtime, settings: WarpSettings)-> () {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), settings.port);
    let static_dir = ".";
    if let Some(init_fn) = settings.init_fn {
        init_fn(rt, socket_addr, static_dir.to_string());
    } else {
        rt.spawn(async move {
            let route_static = dir(static_dir)
                .map(add_headers);
            warp::serve(route_static)
                .run(socket_addr)
                .await;        
        });
    };
}