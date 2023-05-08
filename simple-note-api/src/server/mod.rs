pub mod config;
pub mod controller;
pub mod database;
pub mod model;
pub mod utils;

use crate::server::config::NOT_FOUND;
use crate::server::controller::{
    handle_delete_request, handle_get_all_request, handle_get_request, handle_post_request,
    handle_put_request,
};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// handle requests
pub fn handle_request_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /notes") => handle_post_request(r),
                r if r.starts_with("GET /notes/") => handle_get_request(r),
                r if r.starts_with("GET /notes") => handle_get_all_request(r),
                r if r.starts_with("PUT /notes/") => handle_put_request(r),
                r if r.starts_with("DELETE /notes/") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

// start listener
pub fn start_server() -> TcpListener {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    println!("Server started on port 8080");
    listener
}
