use core::fmt;
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
    let ws_root = "./src/html";
    let db_root = "./src/db_html";

    let url_root = "http://127.0.0.1:5050";

    let buf_reader = BufReader::new(&stream); 
    let mut buf_writer = BufWriter::new(&stream);

    let http_request: Vec<_>= buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|x| !x.is_empty())
            .collect();

    let request_line = &http_request[0];
    let request_line_split: Vec<_> = request_line.split(' ').collect();

    println!("Request: {http_request:#?}");
    println!("First line: {request_line:#?}");

    if request_line == "GET / HTTP/1.1" {
        // buf_writer.write(b"<p>Hello there, world!</p>").unwrap();
        // 1. Reading contents for response
        // 2. Writing out status_line
        // 3. Content length
        // 4. Creating Response
        // 5. Writing response to buffer

        // T <- &string
        #[derive(Debug)]
        #[derive(Clone)]
        struct Response<T> {
            status_line: T,
            content: T,
            ctype: T,
            length: T
        }
        
        /* impl<T> fmt::Debug for Response<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Response")
                    .field("status_line", self.status_line)
                    .field("content_length", self.length)
                    .field("content_type", self.ctype)
                    .field("content", self.content)
                    .finish()
            }
        } */

        impl<T> Response<T>
        where T: fmt::Display + std::clone::Clone, 
        {
            fn outgoing_resp_format(&self) -> String {
                return format!(
                    "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                    self.status_line, self.length, self.ctype, self.content
                )  
            } 

            fn ingoing_resp_format(&self) -> Vec<T> {
                return [self.status_line.clone(), self.length.clone(), self.ctype.clone(), self.content.clone()].to_vec();
            }

            /* fn ingoing_resp_format(&self) -> Vec<_> {
                return format!( "{}\r\n
                    Content-Length: {}\r\n
                    Content-Type: {}\r\n
                    {}",
                    self.status_line, self.length, self.ctype, self.content
                )  
            } */
        }



        let contents = fs::read_to_string(format!("{}{}", ws_root, "/index.html")).unwrap();
        let status_line = "HTTP/1.1 200 OK";
        let length = contents.len().to_string();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"     
        );

        let xresponse = Response {
            status_line: "HTTP/1.1 200 OK",
            content: &contents,
            ctype: "text/html",
            length: &length.as_str()
        };

        let fmtd_xresponse = xresponse.outgoing_resp_format();
        println!("Created with custom response type\n{:#?}", xresponse.ingoing_resp_format());

        buf_writer.write_all(xresponse.outgoing_resp_format().as_bytes()).unwrap();
    } else if request_line == "GET /main.js HTTP/1.1"   {
        let contents = fs::read_to_string("./src/main.js").unwrap();
        let status_line = "HTTP/1.1 200 OK";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/javascript\r\n\r\n{contents}"
        );
        buf_writer.write_all(response.as_bytes()).unwrap();
    } else if request_line_split[0] == "PUT" {
        let file_path = request_line_split[1].replace("%20", " ");

        println!("{}", file_path);
        let _ = fs::write(format!("{}{}", db_root, file_path), "some text"); 

        let location = format!("{}{}", url_root, file_path);
        let contents = fs::read_to_string(format!("{}{}", db_root, file_path)).unwrap();
        let status_line = "HTTP/1.1 201 CREATED";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        buf_writer.write_all(response.as_bytes()).unwrap();
    } else {
        let contents = fs::read_to_string(format!("{}{}", ws_root, "/404.html")).unwrap();
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        buf_writer.write_all(response.as_bytes()).unwrap();
    } 
}
