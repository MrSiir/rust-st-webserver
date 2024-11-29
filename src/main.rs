use st_webserver::{Config, ThreadPool};
use std::{
    env, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process,
};

const HTML_FOLDER: &str = "html";

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).unwrap();
    let pool = ThreadPool::new(4);

    println!("Listen on address {addr}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 {
        send_error_response(&mut stream, 400, "Bad Request").unwrap();
        return;
    }

    let route = parts[1];
    let page_path = match route {
        "/" => format!("{}/index.html", HTML_FOLDER),
        other => format!("{}{}", HTML_FOLDER, other),
    };

    if !fs::metadata(&page_path).is_ok() {
        send_error_response(&mut stream, 404, "Not Found").unwrap();
        return;
    }

    let contents = fs::read_to_string(&page_path).unwrap();

    if let Err(e) = send_response(&mut stream, 200, "OK", &contents) {
        eprintln!("Failed to send response: {}", e);
    }
}

fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    message: &str,
    contents: &str,
) -> Result<(), std::io::Error> {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        message,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes())
}

fn send_error_response(
    stream: &mut TcpStream,
    status_code: u16,
    message: &str,
) -> Result<(), std::io::Error> {
    let page_error = format!("{}/{}.html", HTML_FOLDER, status_code);

    let contents = match fs::read_to_string(&page_error) {
        Ok(content) => content,
        Err(_) => format!(
            "<html><body><h1>{} {}</h1><p>{}</p></body></html>",
            status_code, message, "An error occurred"
        ),
    };

    send_response(stream, status_code, message, &contents)
}
