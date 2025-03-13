use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    

    let mut build_response = |path: &str| {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(path).unwrap();
        let length = contents.len();
        
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        
        stream.write_all(response.as_bytes()).unwrap();
        
    };
    if request_line == "GET /login HTTP/1.1" {
        build_response("public/login.html");
    } else {
        build_response("public/index.html");
    }
}