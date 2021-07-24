use std::{net::{IpAddr, Ipv4Addr, SocketAddr}};

use tokio::runtime::Runtime;
use warp::{Filter, fs::dir};

use crate::{app::{AppState, WarpInitData, WarpSettings}, headers::add_headers};

pub fn start(rt: &Runtime, settings: WarpSettings, state: AppState)-> () {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), settings.port);
    let static_dir = ".";
    if let Some(init_fn) = settings.init_fn {
        let data = WarpInitData {
            socket_addr, 
            static_dir: static_dir.to_string(), 
            state
        };
        init_fn(rt, data);
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