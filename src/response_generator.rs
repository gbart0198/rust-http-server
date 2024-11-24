// 1. Create status line (HTTP/1.1 200 OK)
// 2. Add appropriate headers
// 3. Include response body
// 4. Send response back to client

use std::collections::HashMap;
use std::io::Write;

pub struct HttpResponse {
    version: String,
    headers: HashMap<String, String>,
    status_code: String,
    body: String,
}

impl HttpResponse {
    pub fn new(
        version: String,
        headers: HashMap<String, String>,
        status_code: String,
        body: String,
    ) -> Self {
        HttpResponse { 
            version,
            headers,
            status_code,
            body
        }
    }
}

pub struct HttpResponseGenerator;

impl HttpResponseGenerator {
    pub fn write_response(buf: &mut Vec<u8>, response: HttpResponse) {
        writeln!(buf, "{} {}", response.version, response.status_code).unwrap();
        for (key, val) in response.headers.into_iter() {
                writeln!(buf, "{}: {}", key, val).unwrap();
        }
        writeln!(buf, "").unwrap();
        writeln!(buf, "{}", response.body).unwrap();
    }
}
