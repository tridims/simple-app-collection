mod server;
use server::config::DB_URL;
use server::database::set_database;
use server::{handle_request_stream, start_server};

#[macro_use]
extern crate serde_derive;

fn main() {
    // Set Database
    if let Err(_) = set_database(DB_URL) {
        println!("Error setting database");
        return;
    }

    // start server and print port
    let listener = start_server();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request_stream(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
