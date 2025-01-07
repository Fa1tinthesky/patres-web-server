use std::{
    fs,
    io::{prelude::*, BufReader, BufWriter}, 
    net::{TcpListener, TcpStream}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5050").unwrap();

    /* println!("{:?}", listener);
     */
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("{:?}", stream);
        handle_connection(stream);
    }
    /*
    for stream in listener */ 
}

fn handle_connection(mut stream: TcpStream) {   
    // &stream: bytes
    let buf_reader = BufReader::new(&stream); 
    let mut buf_writer = BufWriter::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // println!("Request: {http_request:#?}");

    if (request_line == "GET / HTTP/1.1") {
        // buf_writer.write(b"<p>Hello there, world!</p>").unwrap();
        let contents = fs::read_to_string("./src/html/index.html").unwrap();
        let status_line = "HTTP/1.1 200 OK";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"     
        );
        buf_writer.write_all(response.as_bytes()).unwrap();
    } else {
        let contents = fs::read_to_string("./src/html/404.html").unwrap();
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        buf_writer.write_all(response.as_bytes()).unwrap();
    } 
}
