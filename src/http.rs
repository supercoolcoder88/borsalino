use crate::http_method::HttpMethod;
use crate::parser;
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Default)]
pub struct Router {
    routes: HashMap<(HttpMethod, String), fn(&mut ResponseWriter)>,
}

impl Router {
    pub fn listen_at(&self, addr: String) {
        if self.routes.is_empty() {
            println!("warning: no configured routes, use self.add_route(method, path, handler");
        }

        println!("listening at {addr}");

        let listener = TcpListener::bind(&addr).expect("failed to bind addr");

        for stream in listener.incoming() {
            let stream = stream.expect("error reading tcp stream");

            match self.handle_client(stream) {
                Ok(_) => continue,
                Err(error) => {
                    eprintln!("error found {error}");
                }
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) -> Result<(), RequestError> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let read_string = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut read_lines = read_string.lines();

        // read request line
        let request_line = read_lines.next().ok_or(parser::ParseError::Line)?;
        let (method, path) = parser::parse_request_line(request_line)?;

        let mut _content_length = 0;

        for line in read_lines {
            // assuming that the first \r\n is end of header lines
            if line.is_empty() {
                break;
            }
            let (header_title, header_val) = parser::parse_header_line(line)?;

            if header_title == "Content-Length" {
                match header_val.parse::<usize>() {
                    Ok(val) => _content_length = val,
                    Err(error) => return Err(RequestError::General(error.to_string())),
                }
            }
        }

        // TODO: Write response

        let mut writer = ResponseWriter::new();

        match self.routes.get(&(method, path)) {
            Some(handler) => handler(&mut writer),
            None => {
                writer.write("HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found");
                return Err(RequestError::General(String::from("fix error handling"))); // TODO: Fix error handling
            }
        }

        stream.write_all(writer.write_buffer.as_slice())?;
        stream.flush()?;

        Ok(())
    }

    pub fn add_route(
        &mut self,
        method: HttpMethod,
        path: String,
        handler: fn(w: &mut ResponseWriter),
    ) {
        self.routes.insert((method, path), handler);
    }
}

pub struct ResponseWriter {
    write_buffer: [u8; 1024],
}

impl ResponseWriter {
    fn new() -> Self {
        Self {
            write_buffer: [0; 1024],
        }
    }

    pub fn write(&mut self, res: &str) {
        self.write_buffer[0..res.len()].copy_from_slice(res.as_bytes());
    }
}

#[derive(Debug)]
pub enum RequestError {
    General(String),
    IO(std::io::Error),
    Parse(parser::ParseError),
}

impl From<parser::ParseError> for RequestError {
    fn from(value: parser::ParseError) -> Self {
        RequestError::Parse(value)
    }
}

impl From<io::Error> for RequestError {
    fn from(value: io::Error) -> Self {
        RequestError::IO(value)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::General(error) => write!(f, "error: {error}"),
            RequestError::IO(error) => write!(f, "io error: {error}"),
            RequestError::Parse(error) => write!(f, "parsing error: {error}"),
        }
    }
}

impl Error for RequestError {}
