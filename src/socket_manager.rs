// 1. Create and bind a TCP socket
// 2. Listen for incoming connections
// 3. Accept client connections

use std::net::TcpListener;
use std::str;

pub struct SocketHandler {}

impl SocketHandler {
    /// Function to create a `TcpListener` on the address provided.
    pub fn initialize(addr: &str) -> TcpListener {
        let listener = TcpListener::bind(addr).unwrap();
        return listener;
    }
}
