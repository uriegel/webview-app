use chrono::Utc;
use tokio::runtime::Runtime;
use warp::{Filter, Reply, fs::File, http::HeaderValue, hyper::{Body, HeaderMap, Response}};

fn create_headers() -> HeaderMap {
    let mut header_map = HeaderMap::new();
    let now = Utc::now();
    let now_str = now.format("%a, %d %h %Y %T GMT").to_string();
    header_map.insert("Expires", HeaderValue::from_str(now_str.as_str()).unwrap());
    header_map.insert("Server", HeaderValue::from_str("webview-app").unwrap());
    header_map
}

pub fn start(rt: &Runtime, port: u16)-> () {
    rt.spawn(async move {

        fn add_headers(reply: File)->Response<Body> {
            let mut res = reply.into_response();
            let headers = res.headers_mut();
            let header_map = create_headers();
            headers.extend(header_map);
            res
        }

        let route_static = warp::fs::dir(".")
            .map(add_headers);

        let routes = route_static;
    
        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;        
    });
}