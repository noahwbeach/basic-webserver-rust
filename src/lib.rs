use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct WebServer {
    listener: Option<TcpListener>,
    address: Option<String>,
    routes: Option<Vec<Route>>,
    base_dir: Option<String>,
}

#[allow(dead_code)]
impl WebServer {
    pub fn new() -> WebServer {
        Self {
            listener: None,
            address: None,
            routes: None,
            base_dir: Some(String::from(".")),
        }
    }
    pub fn bind(self, address: &str) -> WebServer {
        println!("Binding server to address...");
        Self {
            listener: Some(TcpListener::bind(address).unwrap()),
            address: Some(String::from(address)),
            routes: self.routes,
            base_dir: self.base_dir,
        }
    }
    pub fn register(self, routes: Vec<Route>) -> WebServer {
        println!("Registering routes {:?}", routes);
        Self {
            listener: self.listener,
            address: self.address,
            routes: Some(routes),
            base_dir: self.base_dir,
        }
    }
    pub fn base_dir(self, base_dir: &str) -> WebServer {
        Self {
            listener: self.listener,
            address: self.address,
            routes: self.routes,
            base_dir: Some(String::from(base_dir)),
        }
    }
    pub fn listen(self) {
        println!("Listening at {}", self.address.unwrap());
        println!("Routes: {:?}", self.routes);
        for stream in self.listener.unwrap().incoming() {
            handle_client(stream.unwrap(), self.routes.as_ref().unwrap(), self.base_dir.as_ref().unwrap());
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Route {
    path: String,
    method: String,
}

#[allow(dead_code)]
impl Route {
    pub fn new(path: &str, method: &str) -> Route {
        Self {
            path: path.to_string(),
            method: method.to_string(),
        }
    }
}

fn handle_client(mut stream: TcpStream, routes: &Vec<Route>, base_dir: &String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_raw = String::from_utf8_lossy(&buffer[..]);
    let request_vec: Vec<&str> = request_raw.split("\r\n").collect();
    let header: Vec<&str> = request_vec[0].split(" ").collect();
    let method = header[0];
    let path = header[1];

    let mut contents = fs::read_to_string(format!("{}/404.html", base_dir)).unwrap();

    for route in routes {
        if route.method == method && route.path == path {
            if route.path == String::from("/") {
                contents = fs::read_to_string(format!("{}/index.html", base_dir)).unwrap();
                break;
            } else {
                contents = fs::read_to_string(format!("{}{}.html", base_dir, path)).unwrap();
                break;
            }
        }
    }

    println!("{:?}", header);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
