use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn main() {
    bind_port();
}

fn bind_port() {
    let connection = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in connection.incoming() {
        let tcp_stream = stream.unwrap();

        handle_connection(tcp_stream);
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut tcp_stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let req_line: Vec<_> = request_line.split(' ').collect();

    let mut filepath = req_line[1];

    if filepath == "/" {
        filepath = "index.html";
    }

    filepath = match filepath.strip_prefix('/') {
        Some(fp) => fp,
        None => filepath,
    };

    let path = Path::new(&filepath);

    if path.is_file() {
        println!("Serving \"{filepath}\"");
        let status_line = "HTTP/1.1 200 OK";
        let mut contents = fs::read(filepath).unwrap();
        let contents_len = contents.len();

        let msg = format!("{status_line}\r\nContent-Length: {contents_len}\r\n\r\n");
        let mut response = msg.as_bytes().to_vec();
        response.append(&mut contents);

        tcp_stream.write_all(&response).unwrap();
    } else {
        println!("Requested resource not found \"{filepath}\"");
    }
}
