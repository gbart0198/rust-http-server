// 1. Create and bind a TCP socket
// 2. Listen for incoming connections
// 3. Accept client connections

use std::str;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct HttpRequest {
    request_info: String,
    headers: String,
    content: String
}

impl HttpRequest {
    pub fn new(request_info: String, headers: String, content: String) -> Self {
        HttpRequest {
            request_info,
            headers,
            content
        }
    }
}

fn handle_http_request(mut stream: TcpStream) -> HttpRequest {
    let mut buf = [0; 4096];
    let num_read = &stream.read(&mut buf).unwrap();
    let first_line = str::from_utf8(&buf[..*num_read]).unwrap().trim().to_owned();

    let mut headers = String::new();
    let mut content_length = 0;
    loop {
        let num_read = &stream.read(&mut buf).unwrap();
        let header_line = str::from_utf8(&buf[..*num_read]).unwrap();
        if header_line != "\r\n" {
            if header_line.contains("Content-Length") {
                let ct: Vec<_> = header_line.split(":").collect();
                content_length = ct[ct.len() - 1].trim().parse::<usize>().unwrap();
            }
            headers.insert_str(headers.len(), header_line);
        } else {
            break;
        }
    }
    let _ = &stream.read_exact(&mut buf[..content_length + 1]).unwrap();
    let content = str::from_utf8(&buf[..content_length + 1]).unwrap().to_owned();

    return HttpRequest::new(first_line, headers, content)
}

pub struct SocketHandler {}

impl SocketHandler {
    pub fn initialize() {
        let addr = "localhost:8001";
        let listener = TcpListener::bind(addr).unwrap();

        println!("[!!!] Listening on {addr}");
        for stream in listener.incoming() {
            let res = handle_http_request(stream.unwrap());
            println!("Res: {:?}", res)
        }
    }
}
