use super::IPv4Address;
use crate::ip::{InvalidCIDRError, InvalidPrefixError, IPCIDR};
use std::{cmp::Ordering, str::FromStr};

/// An IPv4 Classless Inter-Domain Routing (CIDR) address
#[derive(Clone, Copy, PartialEq, Eq, Ord, Hash)]
pub struct IPv4CIDR {
    address: IPv4Address,
    prefix: u8,
}

impl IPv4CIDR {
    /// Creates a new [`IPv4CIDR`]
    ///
    /// ## Parameters
    ///  * `address` - The address for the CIDR address
    ///  * `prefix` - The subnet prefix length, must be 32 or less
    ///
    /// ## Return Value
    /// Returns the newly created [`IPv4CIDR`]
    pub const fn new(address: IPv4Address, prefix: u8) -> Self {
        assert!(prefix <= 32);
        unsafe { IPv4CIDR::new_unchecked(address, prefix) }
    }

    /// Attempts to create a new [`IPv4CIDR`]
    ///
    /// ## Parameters
    ///  * `address` - The address for the CIDR address
    ///  * `prefix` - The subnet prefix length
    ///
    /// ## Return Value
    /// Returns the newly created [`IPv4CIDR`] if the prefix passed is valid
    /// otherwise.
    pub const fn try_new(address: IPv4Address, prefix: u8) -> Result<Self, InvalidPrefixError> {
        if prefix > 32 {
            Err(InvalidPrefixError { prefix })
        } else {
            Ok(unsafe { IPv4CIDR::new_unchecked(address, prefix) })
        }
    }

    /// Creates a new [`IPv4CIDR`] without checking the prefix
    ///
    /// ## Parameters
    ///  * `address` - The address for the CIDR address
    ///  * `prefix` - The subnet prefix length, must be 32 or less
    ///
    /// ## Return Value
    /// Returns the newly created [`IPv4CIDR`]
    pub const unsafe fn new_unchecked(address: IPv4Address, prefix: u8) -> Self {
        IPv4CIDR { address, prefix }
    }

    /// Gets the address of this CIDR
    ///
    /// ## Return Value
    /// Returns the address of this CIDR
    pub const fn address(&self) -> IPv4Address {
        self.address
    }

    /// Gets the subnet prefix length of this CIDR
    ///
    /// ## Return Value
    /// Returns the subnet prefix length of this CIDR
    pub const fn prefix(&self) -> u8 {
        self.prefix
    }

    /// Sets the address for this CIDR
    ///
    /// ## Parameters
    ///  * `address` - The new address
    pub fn set_address(&mut self, address: IPv4Address) {
        self.address = address;
    }

    /// Sets the prefix for this CIDR
    ///
    /// ## Parameters
    ///  * `prefix` - The new prefix
    pub fn set_prefix(&mut self, prefix: u8) {
        assert!(prefix <= 32);
        self.prefix = prefix;
    }
}

impl Into<IPv4Address> for IPv4CIDR {
    fn into(self) -> IPv4Address {
        self.address
    }
}

impl Into<(IPv4Address, u8)> for IPv4CIDR {
    fn into(self) -> (IPv4Address, u8) {
        (self.address, self.prefix)
    }
}

impl From<IPv4Address> for IPv4CIDR {
    fn from(address: IPv4Address) -> Self {
        IPv4CIDR::new(address, 32)
    }
}

impl<T: Into<IPv4Address>> TryFrom<(T, u8)> for IPv4CIDR {
    type Error = InvalidPrefixError;

    fn try_from(value: (T, u8)) -> Result<Self, Self::Error> {
        IPv4CIDR::try_new(value.0.into(), value.1)
    }
}

impl FromStr for IPv4CIDR {
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

        IPv4CIDR::try_new(address, prefix).map_err(InvalidCIDRError::InvalidPrefix)
    }
}

impl std::fmt::Display for IPv4CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix)
    }
}

impl std::fmt::Debug for IPv4CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl PartialOrd for IPv4CIDR {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.address.partial_cmp(&other.address)
    }
}

impl PartialEq<IPCIDR> for IPv4CIDR {
    fn eq(&self, other: &IPCIDR) -> bool {
        match other {
            IPCIDR::V4(other) => self.eq(other),
            IPCIDR::V6(_) => false,
        }
    }
}

impl PartialOrd<IPCIDR> for IPv4CIDR {
    fn partial_cmp(&self, other: &IPCIDR) -> Option<Ordering> {
        match other {
            IPCIDR::V4(other) => self.partial_cmp(other),
            IPCIDR::V6(_) => Some(Ordering::Less),
        }
    }
}
