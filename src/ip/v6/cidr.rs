use super::IPv6Address;
use crate::ip::{InvalidCIDRError, InvalidPrefixError, IPCIDR};
use std::{cmp::Ordering, str::FromStr};

/// An IPv6 Classless Inter-Domain Routing (CIDR) address
#[derive(Clone, Copy, PartialEq, Eq, Ord, Hash)]
pub struct IPv6CIDR {
    address: IPv6Address,
    prefix: u8,
}

impl IPv6CIDR {
    /// Creates a new [`IPv6CIDR`]
    ///
    /// ## Parameters
    ///  * `address` - The address for the CIDR address
    ///  * `prefix` - The subnet prefix length, must be 128 or less
    ///
    /// ## Return Value
    /// Returns the newly created [`IPv6CIDR`]
    pub const fn new(address: IPv6Address, prefix: u8) -> Self {
        assert!(prefix <= 128);
        unsafe { IPv6CIDR::new_unchecked(address, prefix) }
    }

    /// Attempts to create a new [`IPv6CIDR`]
    ///
    /// ## Parameters
    ///  * `address` - The address for the CIDR address
    ///  * `prefix` - The subnet prefix length
    ///
    /// ## Return Value
    /// Returns the newly created [`IPv6CIDR`] if the prefix passed is valid
    /// otherwise.
    pub const fn try_new(address: IPv6Address, prefix: u8) -> Result<Self, InvalidPrefixError> {
        if prefix > 128 {
            Err(InvalidPrefixError { prefix })
        } else {
            Ok(unsafe { IPv6CIDR::new_unchecked(address, prefix) })
        }
    }

    /// Creates a new [`IPv6CIDR`] without checking the prefix
    ///
    /// ## Parameters
    ///  * `address` - The address for the CIDR address
    ///  * `prefix` - The subnet prefix length, must be 128 or less
    ///
    /// ## Return Value
    /// Returns the newly created [`IPv6CIDR`]
    pub const unsafe fn new_unchecked(address: IPv6Address, prefix: u8) -> Self {
        IPv6CIDR { address, prefix }
    }

    /// Gets the address of this CIDR
    ///
    /// ## Return Value
    /// Returns the address of this CIDR
    pub const fn address(&self) -> IPv6Address {
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
    pub fn set_address(&mut self, address: IPv6Address) {
        self.address = address;
    }

    /// Sets the prefix for this CIDR
    ///
    /// ## Parameters
    ///  * `prefix` - The new prefix
    pub fn set_prefix(&mut self, prefix: u8) {
        assert!(prefix <= 128);
        self.prefix = prefix;
    }
}

impl Into<IPv6Address> for IPv6CIDR {
    fn into(self) -> IPv6Address {
        self.address
    }
}

impl Into<(IPv6Address, u8)> for IPv6CIDR {
    fn into(self) -> (IPv6Address, u8) {
        (self.address, self.prefix)
    }
}

impl From<IPv6Address> for IPv6CIDR {
    fn from(address: IPv6Address) -> Self {
        IPv6CIDR::new(address, 128)
    }
}

impl<T: Into<IPv6Address>> TryFrom<(T, u8)> for IPv6CIDR {
    type Error = InvalidPrefixError;

    fn try_from(value: (T, u8)) -> Result<Self, Self::Error> {
        IPv6CIDR::try_new(value.0.into(), value.1)
    }
}

impl FromStr for IPv6CIDR {
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

        IPv6CIDR::try_new(address, prefix).map_err(InvalidCIDRError::InvalidPrefix)
    }
}

impl std::fmt::Display for IPv6CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix)
    }
}

impl std::fmt::Debug for IPv6CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl PartialOrd for IPv6CIDR {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.address.partial_cmp(&other.address)
    }
}

impl PartialEq<IPCIDR> for IPv6CIDR {
    fn eq(&self, other: &IPCIDR) -> bool {
        match other {
            IPCIDR::V4(_) => false,
            IPCIDR::V6(other) => self.eq(other),
        }
    }
}

impl PartialOrd<IPCIDR> for IPv6CIDR {
    fn partial_cmp(&self, other: &IPCIDR) -> Option<Ordering> {
        match other {
            IPCIDR::V4(_) => Some(Ordering::Greater),
            IPCIDR::V6(other) => self.partial_cmp(other),
        }
    }
}
