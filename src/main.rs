use http_server::{
    socket_manager::SocketHandler,
    request_parser,
    response_generator,
    request_handler
};

fn main() {
    SocketHandler::initialize();
}
