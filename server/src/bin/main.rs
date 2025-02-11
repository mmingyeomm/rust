use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread::{self, Thread}, time::Duration
};

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4); 

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));

    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();


    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();

        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        thread::sleep(Duration::from_secs(5));

        stream.write_all(response.as_bytes()).unwrap();
        println!("response sent");

    } else {

        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        thread::sleep(Duration::from_secs(5));

        stream.write_all(response.as_bytes()).unwrap();
        println!("response sent");
        // some other request
    }
}
