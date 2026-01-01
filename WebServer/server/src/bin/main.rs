use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8181").unwrap(); 

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);

    // 1. Precise Routing & Correct Content-Types
    let (status_line, filename, content_type) = if request.starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "index.html", "text/html")
    } else if request.starts_with("GET /style.css HTTP/1.1") {
        ("HTTP/1.1 200 OK", "style.css", "text/css")
    } else if request.starts_with("GET /images/rusty.png HTTP/1.1") {
        ("HTTP/1.1 200 OK", "images/rusty.png", "image/png")
    } else if request.starts_with("GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "sleep.html", "text/html")
    } else if request.starts_with("GET /images/dexter-2025.jpeg HTTP/1.1") {
        ("HTTP/1.1 200 OK", "images/dexter-2025.jpeg", "image/jpeg")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html")
    };

    // 2. Use fs::read for binary compatibility (images + html)
    let contents = fs::read(filename).unwrap_or_else(|_| {
        b"404 Not Found".to_vec()
    });

    // 3. Construct response with raw bytes
    let response_header = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line, content_type, contents.len()
    );

    stream.write_all(response_header.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap(); // Send raw binary data
    stream.flush().unwrap();
}