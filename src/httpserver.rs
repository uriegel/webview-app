use core::str;
use std::{io::{BufRead, BufReader, BufWriter, Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};

use include_dir::Dir;

use crate::{html, threadpool::{RequestCallback, ThreadPool}};

#[derive(Clone)]
pub struct HttpServer {
    pub port: u32
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
    pub fn run(&self, webroot: Option<Arc<Mutex<Dir<'static>>>>, on_request: Option<RequestCallback>) {
        run(self.port, webroot, on_request);
    }    
}

fn run(port: u32, webroot: Option<Arc<Mutex<Dir<'static>>>>, on_request: Option<RequestCallback>) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::new(8, Arc::new(Mutex::new(on_request)));
    thread::spawn(move || for stream in listener.incoming() {
        let webroot = webroot.clone();
        if let Ok(stream) = stream {
            pool.execute(move|on_request| {
                handle_connection(stream, webroot, on_request);     
            });
        } else {
            break;
        }
    });
}

fn handle_connection(stream: TcpStream, webroot: Option<Arc<Mutex<Dir<'static>>>>, on_request: Arc<Mutex<Option<RequestCallback>>>) {
    stream.set_nodelay(true).unwrap(); // disables Nagle algorithm
    loop {
        let mut buf_reader = BufReader::new(&stream);
        let buf_writer = BufWriter::new(&stream);
        
        let mut headers: Vec<String> = Vec::new();
        loop {
            let mut str = String::new();
            buf_reader.read_line(&mut str).unwrap();
            str = str.trim().to_string();
            if str.len() == 0 {
                break;
            }
            headers.push(str);
        }

        if headers.len() == 0  { 
            return 
        }
        let request_line = &headers[0];

        if request_line.starts_with("GET") {
            route_get(buf_writer, request_line, webroot.clone());
        } else if request_line.starts_with("POST")  {
            route_post(buf_writer, buf_reader, request_line, headers.as_slice(), on_request.clone());
        } else {
            route_not_found(buf_writer);
        }
    }
}    

fn route_get(writer: BufWriter<&TcpStream>, request_line: &String, webroot: Option<Arc<Mutex<Dir<'static>>>>) {
    let pos = request_line[4..].find(" ").unwrap_or(0);
    let path = request_line[4..pos + 4].to_string();

    match (webroot, path) {
        (Some(webroot), path) if path.starts_with("/webroot") =>
            route_get_webroot(writer, &path[9..], webroot),
        (_, _) => route_not_found(writer)
    };
}

fn route_post(writer: BufWriter<&TcpStream>, mut reader: BufReader<&TcpStream>, request_line: &String, headers: &[String], on_request: Arc<Mutex<Option<RequestCallback>>>) {
    let pos = request_line[15..].find(" ").unwrap_or(0);
    let method = request_line[15..pos + 15].to_string();
    let content_length = headers.iter().find_map(|header| {
        if header.starts_with("Content-Length") {
            Some(header[16..].parse::<usize>().unwrap())
        } else {
            None
        }
    }).unwrap_or(0);

    let mut payload: Vec<u8> =  vec![0; content_length];
    reader.read_exact(&mut payload).unwrap();
    let payload= str::from_utf8(payload.as_slice()).unwrap_or("");

    let mut on_request = on_request.lock().unwrap();
    if let Some(on_request) = on_request.take() {
        let res = on_request(&method, payload);
        let json = crate::request::get_output(&res);
        send_json(writer, &json, "HTTP/1.1 200 OK");
    } else {
        route_not_found(writer);
    }
}

fn route_get_webroot(writer: BufWriter<&TcpStream>, path: &str, webroot: Arc<Mutex<Dir<'static>>>) {
    match webroot
            .lock()
            .unwrap()
            .get_file(path) 
            .map(|file| file.contents()) {
        Some(bytes) => {
            send_html_bytes(writer, bytes, "HTTP/1.1 200 OK");
        },
        None => route_not_found(writer)
    };    
}

fn route_not_found(writer: BufWriter<&TcpStream>) {
    send_html(writer, &html::not_found(), "HTTP/1.1 404 NOT FOUND"); 
}

fn send_html(mut writer: BufWriter<&TcpStream>, html: &str, status_line: &str) {
    let length = html.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html}");
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn send_json(mut writer: BufWriter<&TcpStream>, json: &str, status_line: &str) {
    let length = json.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{json}");
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn send_html_bytes(mut writer: BufWriter<&TcpStream>, html: &[u8], status_line: &str) {
    let length = html.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    writer.write_all(response.as_bytes()).unwrap();
    writer.write_all(html).unwrap();
    writer.flush().unwrap();
}