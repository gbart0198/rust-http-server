use crate::{request_parser::HttpRequest, response_generator::HttpResponse};

pub fn handle_get_item(request: HttpRequest) -> HttpResponse {
    println!("IN GET ITEM");
    println!("{:?}", request);
    return HttpResponse::new(200)
}

pub fn handle_index(request: HttpRequest) -> HttpResponse {
    println!("IN INDEX");
    println!("{:?}", request);
    return HttpResponse::new(200)
}

pub fn handle_segment(request: HttpRequest) -> HttpResponse {
    println!("IN HANDLE SEGMENT");
    println!("{:?}", request);
    return HttpResponse::new(200)
}


