use chrono::prelude::*;

use hermes::ThreadPool;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();

    let port = match args.next() {
        Some(port) => str::parse::<i32>(&port).expect("Invalid port number"),
        None => 8000,
    };
    let bind_addr = format!("0.0.0.0:{port}");

    println!(
        r#"
 __                                                 
/\ \                                                
\ \ \___      __   _ __    ___ ___      __    ____  
 \ \  _ `\  /'__`\/\`'__\/' __` __`\  /'__`\ /',__\ 
  \ \ \ \ \/\  __/\ \ \/ /\ \/\ \/\ \/\  __//\__, `\
   \ \_\ \_\ \____\\ \_\ \ \_\ \_\ \_\ \____\/\____/
    \/_/\/_/\/____/ \/_/  \/_/\/_/\/_/\/____/\/___/ 
                                                    
                                                    
    Hermes: A concurrent web-server written in Rust
        "#,
    );
    println!("Serving HTTP over port {port} (http://0.0.0.0:{port}/):");

    bind_port(&bind_addr);
}

fn bind_port(bind_addr: &str) {
    let connection = TcpListener::bind(bind_addr).unwrap();

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

    let dt = Local::now().format("%Y-%m-%d %H:%M:%S");
    if path.is_file() {
        println!("[{dt}] \"{request_line}\" 200");
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read(filepath).unwrap();
    } else {
        println!("[{dt}] ERROR \"{request_line}\" 404");
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = "ERROR: Requested resource not found".as_bytes().to_vec();
    }

    let contents_len = contents.len();
    let msg = format!("{status_line}\r\nContent-Length: {contents_len}\r\n\r\n");
    let mut response = msg.as_bytes().to_vec();
    response.append(&mut contents);
    tcp_stream.write_all(&response).unwrap();
}
