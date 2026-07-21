use std::str::FromStr;
use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
    Query,
}

impl FromStr for HttpMethod {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            "QUERY" => Ok(HttpMethod::Query),
            _ => Err(ParseError::Method),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Line,
    Method,
    Path,
    Version,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Line => write!(f, "invalid line"),
            ParseError::Method => write!(f, "invalid method"),
            ParseError::Path => write!(f, "invalid path"),
            ParseError::Version => write!(f, "invalid version"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_request_line(line: &str) -> Result<(HttpMethod, String), ParseError> {
    let line = line.trim();

    let mut parts = line.split_whitespace();

    let method: HttpMethod = parts.next().ok_or(ParseError::Line)?.parse()?;

    let path = parts.next().ok_or(ParseError::Path)?;

    let ver = parts.next().ok_or(ParseError::Version)?;

    // Only allow HTTP/1.1
    if ver.ne("HTTP/1.1") {
        return Err(ParseError::Version);
    }

    Ok((method, path.to_string()))
}
