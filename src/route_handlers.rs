use crate::{request_parser::HttpRequest, response_generator::HttpResponse};
use std::{collections::HashMap, time::SystemTime};

pub struct Routes {

}

impl Routes {
    pub fn handle_get_item(request: HttpRequest) -> HttpResponse {
        let mut headers : HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Date"), format!("{:?}", SystemTime::now()));
        headers.insert(String::from("Accept"), String::from("*/*"));
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Content-Type"), String::from("text/html"));

        let body = String::from("<!DOCTYPE html><html><p>Handle Get Item</p></html>");
        return HttpResponse::new(request.version, headers, String::from("200 OK"), body)
    }

    pub fn handle_index(request: HttpRequest) -> HttpResponse {
        let mut headers : HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Date"), format!("{:?}", SystemTime::now()));
        headers.insert(String::from("Accept"), String::from("*/*"));
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Content-Type"), String::from("text/html"));

        let body = String::from("<!DOCTYPE html><html><p>Handle Index Page</p></html>");
        return HttpResponse::new(request.version, headers, String::from("200 OK"), body)
    }

    pub fn handle_segment(request: HttpRequest) -> HttpResponse {
        let mut headers : HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Date"), format!("{:?}", SystemTime::now()));
        headers.insert(String::from("Accept"), String::from("*/*"));
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Content-Type"), String::from("text/html"));

        let body = String::from("<!DOCTYPE html><html><p>Handle Segment Page</p></html>");
        return HttpResponse::new(request.version, headers, String::from("200 OK"), body)
    }
}



