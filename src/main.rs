use http_server::{
    request_handler::HttpHandler,
    request_parser::{self, HttpRequestParser},
    response_generator, routes,
    socket_manager::SocketHandler,
};

fn main() {
    let listener = SocketHandler::initialize("localhost:8001");
    let mut handler = HttpHandler::new();
    handler.register_route(String::from(""), routes::handle_index).unwrap();
    handler
        .register_route(String::from("test"), routes::handle_get_item)
        .unwrap();
    handler
        .register_route(String::from("segment"), routes::handle_segment)
        .unwrap();

    for stream in listener.incoming() {
        let request = request_parser::HttpRequestParser::handle_http_request(stream.unwrap());
        println!("Request type: {}", request.request_type);
        println!("Request route: {}", request.route);
        println!("Request version: {}", request.version);
        println!("Request headers: {:?}", request.headers);
        println!("Request content: {}", request.content);

        handler.handle_request(request);
    }
}
