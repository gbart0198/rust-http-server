// 1. Route requests based on path
// 2. Process different HTTP methods
// 3. Handle errors and generate appropriate responses

use crate::{request_parser::HttpRequest, response_generator::HttpResponse};
use std::{collections::HashMap, error::Error};


/// Contains methods to register routes and handle an `HttpRequest` object,
/// executing the mapped method with the `HttpRequest` as an argument.
pub struct HttpHandler {
    pub routes: HashMap<String, fn(HttpRequest)>
}

impl HttpHandler {

    pub fn new() -> Self {
        HttpHandler {
            routes: HashMap::new()
        }
    }

    pub fn register_route(&mut self, route: String, func: fn(HttpRequest)) -> Result<(), Box<dyn Error>> {
        self.routes.insert(route, func);
        return Ok(())
    }

    pub fn handle_request(&self, request: HttpRequest) {
        let full_route = request.route.to_string();
        let mut route = "/";
        let parts: Vec<&str> = full_route.split("/").collect();
        if parts.len() > 1 {
            // requesting something other than index
            route = parts[1];
        }
        match self.routes.get(route) {
            Some(func) => func(request),
            None => println!("Func not found for route {route}")
        }
    }
}
