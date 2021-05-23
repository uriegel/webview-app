
//! This module contains all methods to add additional HTTP headers or automatically 
//! create default headers.

use chrono::Utc;
use warp::{Reply, fs::File, http::HeaderValue, hyper::{Body, HeaderMap, Response}};

/// Add this method in WarpSettings::init_fn for the static route, if you have implemented custom warp route filters
/// 
/// Code:
///
/// ``` 
/// fn server(rt: &Runtime, socket_addr: SocketAddr, static_dir: String) {
///     rt.spawn(async move {
///         let route_static = dir(static_dir)
///             .map(add_headers);
///     ...
///         warp::serve(routes)
///             .run(socket_addr)
///             .await;        
/// });
///
///     ...
///     warp_settings: Some(WarpSettings { 
///         port: 9999,
///         init_fn: Some(server),
///     }
/// ``` 
pub fn add_headers(reply: File)->Response<Body> {
    let mut res = reply.into_response();
    let headers = res.headers_mut();
    let header_map = create_headers();
    headers.extend(header_map);
    res
}

fn create_headers() -> HeaderMap {
    let mut header_map = HeaderMap::new();
    let now = Utc::now();
    let now_str = now.format("%a, %d %h %Y %T GMT").to_string();
    header_map.insert("Expires", HeaderValue::from_str(now_str.as_str()).unwrap());
    header_map.insert("Server", HeaderValue::from_str("webview-app").unwrap());
    header_map
}
