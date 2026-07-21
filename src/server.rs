use crate::parser;
use std::{
    error::Error,
    fmt,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub enum RequestError {
    Io(io::Error),
    Parse(parser::ParseError),
}

impl From<io::Error> for RequestError {
    fn from(value: io::Error) -> Self {
        RequestError::Io(value)
    }
}
impl From<parser::ParseError> for RequestError {
    fn from(value: parser::ParseError) -> Self {
        RequestError::Parse(value)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Io(error) => write!(f, "io error: {error}"),
            RequestError::Parse(error) => write!(f, "parsing error: {error}"),
        }
    }
}

impl Error for RequestError {}

pub fn listen_at(addr: String) {
    println!("listening at {addr}");

    let listener = TcpListener::bind(&addr).expect("failed to bind addr");

    for stream in listener.incoming() {
        let stream = stream.expect("error reading tcp stream");

        match handle_client(stream) {
            Ok(_) => continue,
            Err(error) => {
                eprintln!("error found {error}");
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), RequestError> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let read_string = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut read_lines = read_string.lines();

    // println!("value read:\n{read_string}");

    // read request line
    let request_line = read_lines.next().ok_or(parser::ParseError::Line)?;

    // TODO: Remove logs
    let (method, path) = parser::parse_request_line(request_line)?;
    println!("method{:?}, path: {path}", method);
    for line in read_lines {
        // assuming that the first \r\n is end of header lines
        if line.is_empty() {
            break;
        }
        let (header_title, header_val) = parser::parse_header_line(line)?;
        println!("title: {header_title}\n val: {header_val}");
    }
    // Write response
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
