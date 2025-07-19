use crate::board::*;

use std::net::TcpStream;
use std::io::Read;

pub fn handle_connection(stream: &mut TcpStream) -> Option<[[Square; 8]; 8]> {
    let mut response = Vec::new();
    if let Err(e) = stream.read_to_end(&mut response) {
        eprintln!("Failed to read response: {}", e);
        return None;
    }

    match serde_json::from_slice::<[[Square; 8]; 8]>(&response) {
        Ok(board) => Some(board),
        Err(e) => {
            eprintln!("Failed to parse board: {}", e);
            None
        }
    }
}


pub fn connect() -> Option<TcpStream> {
    let mut stream: Option<TcpStream> = None;

    match TcpStream::connect("127.0.0.1:7878") {
        Ok(s) => stream = Some(s),
        Err(e) => eprintln!("Connection failed: {}", e),
    }

    stream
}