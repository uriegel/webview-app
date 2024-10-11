use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};

use include_dir::Dir;

use crate::{html, threadpool::ThreadPool};

pub struct HttpServer {
    pub port: u32,
}

pub struct HttpServerBuilder {
    port: u32
}

impl HttpServerBuilder {
    pub fn new()->Self {
        HttpServerBuilder { port: 7000 }
    }
        
    pub fn port(mut self, val: u32)->Self {
        self.port = val;
        self
    }

    pub fn build(&self)->HttpServer {
        HttpServer {
            port: self.port
        }
    }
}

impl HttpServer {
    pub fn run(&self, webroot: Option<Arc<Mutex<Dir<'static>>>>) {
        run(self.port, webroot);
    }    
}

fn run(port: u32, webroot: Option<Arc<Mutex<Dir<'static>>>>) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::new(8);
    thread::spawn(move || for stream in listener.incoming() {
        let webroot = webroot.clone();
        if let Ok(stream) = stream {
            pool.execute(move|| {
                handle_connection(stream, webroot);     
            });
        } else {
            break;
        }
    });
}

fn handle_connection(mut stream: TcpStream, webroot: Option<Arc<Mutex<Dir<'static>>>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader    
        .lines()
        .next().unwrap_or(Ok("".to_string())).unwrap();

    if request_line.starts_with("GET") {
        route_get(stream, request_line, webroot.clone());
    } else {
        route_not_found(stream);
    }
}    

fn route_get(stream: TcpStream, request_line: String, webroot: Option<Arc<Mutex<Dir<'static>>>>) {
    let pos = request_line[4..].find(" ").unwrap_or(0);
    let path = request_line[4..pos + 4].to_string();
    println!("{}", path);

    match (webroot, path) {
        (Some(webroot), path) if path.starts_with("/webroot") =>
            route_get_webroot(stream, &path[9..], webroot),
        (_, _) => route_not_found(stream)
    };
}

fn route_get_webroot(stream: TcpStream, path: &str, webroot: Arc<Mutex<Dir<'static>>>) {
    match webroot
            .lock()
            .unwrap()
            .get_file(path) 
            .map(|file| file.contents()) {
        Some(bytes) => {
            send_html_bytes(stream, bytes, "HTTP/1.1 200 OK");
        },
        None => route_not_found(stream)
    };    
}

fn route_not_found(stream: TcpStream) {
    send_html(stream, &html::not_found(), "HTTP/1.1 404 NOT FOUND"); 
}

fn send_html(mut stream: TcpStream, html: &str, status_line: &str) {
    let length = html.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nConnection: Close\r\n\r\n{html}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn send_html_bytes(mut stream: TcpStream, html: &[u8], status_line: &str) {
    let length = html.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nConnection: Close\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(html).unwrap();
}