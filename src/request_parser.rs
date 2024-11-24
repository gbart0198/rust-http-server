// 1. Read raw data from client socket
// 2. Parse HTTP request line (method, path, version)
// 3. Parse HTTP headers
// 4. Handle request body (if present)
//

use std::io::{BufRead, BufReader, Read};
use std::{collections::HashMap, net::TcpStream, str};

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
    pub fn handle_http_request(stream: &TcpStream) -> HttpRequest {
        let mut reader = BufReader::new(stream);
        let mut content_length = 0;
        let mut headers = HashMap::new();
        let mut line = String::new();

        reader.read_line(&mut line).unwrap();
        let line_vec: Vec<&str> = line.trim().split(" ").collect();
        println!("{:?}", line_vec);

        assert_eq!(line_vec.len(), 3);

        let request_type = line_vec[0].to_string();
        let route = line_vec[1].to_string();
        let request_version = line_vec[2].to_string();
        let mut content = String::new();

        loop {
            let mut header = String::new();
            reader.read_line(&mut header).unwrap();
            match header.find(":") {
                Some(key_break) => {
                    let key = &header[0..key_break];
                    let val = &header[key_break..header.len()];
                    if header == "Content-Length" {
                        content_length = val.parse().unwrap();
                    }

                    headers.insert(key.to_string(), val.trim().to_string());
                }
                None => {
                    if content_length == 0 {
                        break;
                    }
                    // empty line between header and content
                    let mut bytes_read = 0;
                    while bytes_read < content_length {
                        bytes_read += reader.read_line(&mut header).unwrap();
                        content.push_str(&header);
                    }
                }
            }
        }

        let req = HttpRequest::new(request_type, route, request_version, headers, content);
        println!("{:?}", req);

        req
    }
}
