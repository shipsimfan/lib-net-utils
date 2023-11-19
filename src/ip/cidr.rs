use super::{v4::IPv4CIDR, v6::IPv6CIDR, IPAddress, InvalidCIDRError};
use std::{cmp::Ordering, str::FromStr};

/// An IP Classless Inter-Domain Routing (CIDR) address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IPCIDR {
    /// An IPv4 CIDR
    V4(IPv4CIDR),

    /// An IPv6 CIDR
    V6(IPv6CIDR),
}

impl From<IPv4CIDR> for IPCIDR {
    fn from(cidr: IPv4CIDR) -> Self {
        IPCIDR::V4(cidr)
    }
}

impl From<IPv6CIDR> for IPCIDR {
    fn from(cidr: IPv6CIDR) -> Self {
        IPCIDR::V6(cidr)
    }
}

impl FromStr for IPCIDR {
    type Err = InvalidCIDRError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');

        let address = parts
            .next()
            .ok_or(InvalidCIDRError::MissingAddress)?
            .parse()
            .map_err(InvalidCIDRError::AddressParseError)?;

        let prefix = parts
            .next()
            .ok_or(InvalidCIDRError::MissingPrefix)?
            .parse()
            .map_err(InvalidCIDRError::PrefixParseError)?;

        if parts.next().is_some() {
            return Err(InvalidCIDRError::ExtraContent);
        }

        Ok(match address {
            IPAddress::V4(address) => IPCIDR::V4(
                IPv4CIDR::try_new(address, prefix).map_err(InvalidCIDRError::InvalidPrefix)?,
            ),
            IPAddress::V6(address) => IPCIDR::V6(
                IPv6CIDR::try_new(address, prefix).map_err(InvalidCIDRError::InvalidPrefix)?,
            ),
        })
    }
}

impl std::fmt::Display for IPCIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IPCIDR::V4(cidr) => cidr.fmt(f),
            IPCIDR::V6(cidr) => cidr.fmt(f),
        }
    }
}

impl std::fmt::Debug for IPCIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl PartialEq<IPv4CIDR> for IPCIDR {
    fn eq(&self, other: &IPv4CIDR) -> bool {
        match self {
            IPCIDR::V4(cidr) => cidr.eq(other),
            IPCIDR::V6(_) => false,
        }
    }
}

impl PartialEq<IPv6CIDR> for IPCIDR {
    fn eq(&self, other: &IPv6CIDR) -> bool {
        match self {
            IPCIDR::V4(_) => false,
            IPCIDR::V6(cidr) => cidr.eq(other),
        }
    }
}

impl PartialOrd<IPv4CIDR> for IPCIDR {
    fn partial_cmp(&self, other: &IPv4CIDR) -> Option<Ordering> {
        match self {
            IPCIDR::V4(cidr) => cidr.partial_cmp(other),
            IPCIDR::V6(_) => Some(Ordering::Greater),
        }
    }
}

impl PartialOrd<IPv6CIDR> for IPCIDR {
    fn partial_cmp(&self, other: &IPv6CIDR) -> Option<Ordering> {
        match self {
            IPCIDR::V4(_) => Some(Ordering::Less),
            IPCIDR::V6(cidr) => cidr.partial_cmp(other),
        }
    }
}
