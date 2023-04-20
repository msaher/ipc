use std::fmt;
use std::num::ParseIntError;
use std::net::AddrParseError;

#[derive(Debug, Clone)]
pub struct InvalidPrefixErr;

impl fmt::Display for InvalidPrefixErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid prefix value")
    }
}

