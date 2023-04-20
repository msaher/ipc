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

    pub fn network_address(&self) -> Ipv4Addr {
        let suffix_len: u8 = 32-self.prefix_len;
        let addr: u32 = self.ip.into();
        let mask: u32 = !0 << suffix_len;
        let netadd = addr & mask;
        Ipv4Addr::from(netadd)
    }

    pub fn broadcast_address(&self) -> Ipv4Addr {
        let addr: u32 = self.ip.into();
        let mask: u32 = !0 >> self.prefix_len;
        let broadd = addr | mask;
        Ipv4Addr::from(broadd)
    }

    pub fn ip(&self) -> Ipv4Addr {
        return self.ip;
    }

    pub fn prefix_len(&self) -> u8 {
        return self.prefix_len;
    }
}

impl From<Ipv4Addr> for Ipv4Cidr {
    fn from(ip: Ipv4Addr) -> Self {
        Ipv4Cidr::new(ip, ipv4_class(&ip) as u8).unwrap()
    }
}

impl FromStr for Ipv4Cidr {
    type Err = CidrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/').collect::<Vec<&str>>();
        let ip = Ipv4Addr::from_str(parts[0])?;

        if parts.len() == 1 {
            return Ok(Ipv4Cidr::from(ip));
        }
        else if parts.len() != 2 {
            return Err(Self::Err::TokenErr)
        }

        let prefix_len = u8::from_str(parts[1])?;
        if prefix_len >= 32 || prefix_len == 0 {
            return Err(Self::Err::InvalidErr(InvalidPrefixErr));
        }

        return Ok(Ipv4Cidr {ip, prefix_len});
    }
}
