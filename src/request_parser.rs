// 1. Read raw data from client socket
// 2. Parse HTTP request line (method, path, version)
// 3. Parse HTTP headers
// 4. Handle request body (if present)
//

use std::{collections::HashMap, io::Read, net::TcpStream, str};

#[derive(Debug)]
pub struct HttpRequest {
    pub request_type: String,
    pub route: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub content: String,
}

impl HttpRequest {
    pub fn new(
        request_type: String,
        route: String,
        version: String,
        headers: HashMap<String, String>,
        content: String,
    ) -> Self {
        HttpRequest {
            request_type,
            route,
            version,
            headers,
            content,
        }
    }
}

pub struct HttpRequestParser;

impl HttpRequestParser {
    pub fn handle_http_request(mut stream: &TcpStream) -> HttpRequest {
        let mut buf = [0; 4096];
        let num_read = &stream.read(&mut buf).unwrap();
        let request_info: Vec<&str> = str::from_utf8(&buf[..*num_read])
            .unwrap()
            .trim()
            .split(" ")
            .collect();
        let request_type = request_info[0].to_string();
        let route = request_info[1].to_string();
        let request_version = request_info[2].to_string();

        let mut headers = HashMap::new();
        let mut content_length = 0;
        loop {
            let num_read = &stream.read(&mut buf).unwrap();
            let header_line = str::from_utf8(&buf[..*num_read]).unwrap();
            if header_line != "\r\n" {
                let header_info: Vec<_> = header_line.split(":").collect();
                let key = header_info[0].trim();
                let val = header_info[1].trim();
                if header_line.contains("Content-Length") {
                    content_length = header_info[header_info.len() - 1]
                        .trim()
                        .parse::<usize>()
                        .unwrap();
                }
                headers.insert(key.to_string(), val.to_string());
            } else {
                break;
            }
        }
        let mut content = String::new();
        if content_length > 0 {
            let _ = &stream.read_exact(&mut buf[..content_length + 1]).unwrap();
            content = str::from_utf8(&buf[..content_length + 1])
                .unwrap()
                .to_owned();
        }

        return HttpRequest::new(request_type, route, request_version, headers, content);
    }
}
