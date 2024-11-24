use std::io::Write;

use http_server::{
    request_handler::HttpHandler, request_parser::HttpRequestParser, response_generator::HttpResponseGenerator,
    route_handlers::Routes, socket_manager::SocketHandler,
};

fn main() {
    let listener = SocketHandler::initialize("localhost:8001");
    let mut handler = HttpHandler::new();
    handler
        .register_route(String::from(""), Routes::handle_index)
        .unwrap();
    handler
        .register_route(String::from("item"), Routes::handle_get_item)
        .unwrap();
    handler
        .register_route(String::from("segment"), Routes::handle_segment)
        .unwrap();

    for mut stream in listener.incoming() {
        let request = HttpRequestParser::handle_http_request(stream.as_mut().unwrap());
        let mut response_buffer: Vec<u8> = vec![];
        let response = handler.handle_request(request);
        HttpResponseGenerator::write_response(&mut response_buffer, response);
        stream.unwrap().write_all(&response_buffer).unwrap();
    }
}
