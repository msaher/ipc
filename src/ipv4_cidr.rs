mod error;

use std::net::Ipv4Addr;
use std::convert::{From, Into};
use std::str::FromStr;
use super::ipv4_class::ipv4_class;

use error::{InvalidPrefixErr, CidrParseError};

pub struct Ipv4Cidr {
    ip: Ipv4Addr,
    prefix_len: u8,
}

impl Ipv4Cidr {
    pub fn new(ip: Ipv4Addr, prefix_len: u8) -> Result<Ipv4Cidr, InvalidPrefixErr> {
        if prefix_len >= 32 || prefix_len == 0 {
            return Err(InvalidPrefixErr);
        }

        return Ok(Ipv4Cidr { ip, prefix_len })
    }
    pub fn ip(&self) -> Ipv4Addr {
        return self.ip;
    }

    pub fn prefix_len(&self) -> u8 {
        return self.prefix_len;
    }
}
