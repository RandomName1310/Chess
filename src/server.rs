use crate::board::*;
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};
use serde_json;

pub fn listen_for_requests() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to port");
    println!("Server listening on port 7878");

    if let Ok((stream, addr)) = listener.accept() {
        println!("Connection established with: {}", addr);
        handle_connection(stream);
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    let response = serde_json::to_string(&BOARD_LAYOUT).unwrap();

    loop{
        // send JSON
        if stream.write_all(response.as_bytes()).is_err() {
            eprintln!("Failed to send board data");
        }
    }
}