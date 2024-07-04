use hermes::ThreadPool;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn main() {
    bind_port();
}

fn bind_port() {
    let connection = TcpListener::bind("127.0.0.1:8000").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in connection.incoming() {
        let tcp_stream = stream.unwrap();

        thread_pool.spawn(|| handle_connection(tcp_stream));
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

    let status_line;
    let mut contents;

    if path.is_file() {
        println!("Serving \"{filepath}\"");
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read(filepath).unwrap();
    } else {
        println!("Requested resource not found \"{filepath}\"");
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = "Requested resource not found".as_bytes().to_vec();
    }

    let contents_len = contents.len();
    let msg = format!("{status_line}\r\nContent-Length: {contents_len}\r\n\r\n");
    let mut response = msg.as_bytes().to_vec();
    response.append(&mut contents);
    tcp_stream.write_all(&response).unwrap();
}
