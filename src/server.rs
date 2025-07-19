use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

pub fn listen_for_requests() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("listening");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("connection!!");
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    stream.write_all(status_line.as_bytes()).unwrap();
    println!("response sent");
}