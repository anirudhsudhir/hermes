use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let connection = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in connection.incoming() {
        let tcp_stream = stream.unwrap();

        handle_connection(tcp_stream);
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut tcp_stream);
    // let request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let contents = fs::read_to_string("index.html").unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let contents_len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {contents_len}\r\n\r\n{contents}");

    tcp_stream.write_all(response.as_bytes()).unwrap();
}
