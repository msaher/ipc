use std::net::Ipv4Addr;
use std::fmt::{Display, self};

pub enum Ipv4Class {
    A = 8,
    B = 16,
    C = 24,
    D = 32,
}

pub fn ipv4_class(ip: &Ipv4Addr) -> Ipv4Class {
    use Ipv4Class::*;
    let oct = ip.octets()[0];
    match oct {
        n if n < 127 => A,
        n if n < 191 => B,
        n if n < 223 => C,
        // n if n < 239 => D,
        _ => D
    }
}

impl Display for Ipv4Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let class = match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",

        };
        write!(f, "{}", class)
    }
}
