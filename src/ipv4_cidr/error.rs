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

#[derive(Debug, Clone)]
pub enum CidrParseError {
    TokenErr,
    AddrErr(AddrParseError),
    PrefixErr,
    InvalidErr(InvalidPrefixErr),
}

impl fmt::Display for CidrParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AddrErr(add_err) => add_err.fmt(f),
            Self::TokenErr => write!(f, "must have one / character"),
            Self::PrefixErr => write!(f, "cant find a valid prefix"),
            Self::InvalidErr(e) => e.fmt(f)
        }
    }

}

impl From<AddrParseError> for CidrParseError {
    fn from(e: AddrParseError) -> CidrParseError {
        return CidrParseError::AddrErr(e);
    }
}

impl From<ParseIntError> for CidrParseError {
    fn from(_e: ParseIntError) -> CidrParseError {
        return CidrParseError::PrefixErr;
    }
}

