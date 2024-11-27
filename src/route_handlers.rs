use crate::{request_parser::HttpRequest, response_generator::HttpResponse};
use crate::file_loader::FileLoader;
use std::{collections::HashMap, time::SystemTime};

pub struct Routes {

}

impl Routes {
    pub fn handle_get_item(request: HttpRequest) -> HttpResponse {
        let mut headers : HashMap<String, String> = HashMap::new();
        Routes::add_standard_route_headers(&mut headers);

        let f_loader = FileLoader::new("./content/item.html");
        let file_content = f_loader.read();

        return HttpResponse::new(request.version, headers, String::from("200 OK"), file_content)
    }

    pub fn handle_index(request: HttpRequest) -> HttpResponse {
        let mut headers : HashMap<String, String> = HashMap::new();
        Routes::add_standard_route_headers(&mut headers);

        let f_loader = FileLoader::new("./content/index.html");
        let file_content = f_loader.read();

        return HttpResponse::new(request.version, headers, String::from("200 OK"), file_content)
    }

    pub fn handle_segment(request: HttpRequest) -> HttpResponse {
        let mut headers : HashMap<String, String> = HashMap::new();
        Routes::add_standard_route_headers(&mut headers);

        let f_loader = FileLoader::new("./content/segment.html");
        let file_content = f_loader.read();

        return HttpResponse::new(request.version, headers, String::from("200 OK"), file_content)
    }

    fn add_standard_route_headers(headers: &mut HashMap<String, String>) {
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Date"), format!("{:?}", SystemTime::now()));
        headers.insert(String::from("Accept"), String::from("*/*"));
        headers.insert(String::from("Server"), String::from("Rustify HTTP"));
        headers.insert(String::from("Content-Type"), String::from("text/html"));
    }
}



