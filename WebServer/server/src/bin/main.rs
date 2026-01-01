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

// helper function to set the correct header to the file extension
fn get_content_type(path: &str) -> &'static str {
    if path.ends_with(".html") || path == "/" { "text/html" }
    else if path.ends_with(".css") { "text/css" }
    else if path.ends_with(".png") { "image/png" }
    else if path.ends_with(".jpg") || path.ends_with(".jpeg") { "image/jpeg" }
    else { "text/plain" }
}

// helper function to send the response with the correct content types
fn send_response(mut stream: TcpStream, status: &str, c_type: &str, body: Vec<u8>) {
    let header = format!("{status}\r\nContent-Type: {c_type}\r\nContent-Length: {}\r\n\r\n", body.len());
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(&body);
    let _ = stream.flush();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer)
        .lines()
        .next()
        .unwrap_or("")
        .to_string();

    // Extract the path (middle part of "GET /path HTTP/1.1") (e.g. "GET /images/rusty.png HTTP/1.1")
    let parts: Vec<&str> = request.split_whitespace().collect();
    let path = if parts.len() > 1 { parts[1] } else { "/" };

    if path.contains("..") {
        let contents = fs::read("404.html").unwrap_or_default();
        send_response(stream, "HTTP/1.1 404 NOT FOUND", "text/html", contents);
        return;
    }

    // Dynamic Routing
    let (status_line, filename) = if path == "/" {
        ("HTTP/1.1 200 OK", "index.html")
    } else if path == "/sleep" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "sleep.html")
    } else {
        ("HTTP/1.1 200 OK", &path[1..])
    };

    let content_type = get_content_type(filename);

    match fs::read(filename) {
        Ok(contents) => send_response(stream, status_line, content_type, contents),
        Err(_) => {
            let contents = fs::read("404.html").unwrap_or_default();
            send_response(stream, "HTTP/1.1 404 NOT FOUND", "text/html", contents);
        }
    }
}