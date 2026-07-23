use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
    Query,
}

impl FromStr for HttpMethod {
    type Err = InvalidMethod;

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "QUERY" => Ok(Self::Query),
            _ => Err(InvalidMethod),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidMethod;

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid HTTP method")
    }
}

impl Error for InvalidMethod {}
