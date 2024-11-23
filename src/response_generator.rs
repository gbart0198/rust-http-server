// 1. Create status line (HTTP/1.1 200 OK)
// 2. Add appropriate headers
// 3. Include response body
// 4. Send response back to client

use std::io::Write;

pub struct HttpResponse {
    status_code: usize
}

impl HttpResponse {
    pub fn new(status_code: usize) -> Self {
        HttpResponse {
            status_code
        }
    }
}


pub struct HttpResponseGenerator;


impl HttpResponseGenerator {
    pub fn write_response(buf: &mut Vec<u8>, response: HttpResponse) {
        writeln!(buf, "Status Code: {}", response.status_code).unwrap();
    }
}


