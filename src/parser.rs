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

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Line,
    Method,
    Path,
    Version,
    Header,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Line => write!(f, "invalid line"),
            ParseError::Method => write!(f, "invalid method"),
            ParseError::Path => write!(f, "invalid path"),
            ParseError::Version => write!(f, "invalid version"),
            ParseError::Header => write!(f, "invalid header"),
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

pub fn parse_header_line(line: &str) -> Result<(String, String), ParseError> {
    let line = line.trim();

    let colon = line.find(':').ok_or(ParseError::Header)?;
    Ok((
        line[..colon].to_owned(),
        line[colon + 1..].trim().to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_parse_request_line() {
        let (method, path) = parse_request_line("POST /users HTTP/1.1\r\n").unwrap();
        assert_eq!(method, HttpMethod::Post);
        assert_eq!(path, "/users");
    }

    #[test]
    fn error_bad_method_parse_request_line() {
        let error = parse_request_line("badMeth0D /users HTTP/1.1\r\n").unwrap_err();
        assert_eq!(error, ParseError::Method);
    }

    #[test]
    fn success_parse_header_line() {
        let (header_name, header_val) =
            parse_header_line("   cool-HeadER123: value12:123").unwrap();
        assert_eq!(header_name, "cool-HeadER123");
        assert_eq!(header_val, "value12:123");
    }

    #[test]
    fn error_no_colon_parse_header_line() {
        let error = parse_header_line("  header123 value").unwrap_err();
        assert_eq!(error, ParseError::Header);
    }
}
