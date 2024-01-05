//! MAC address utilites

use std::str::FromStr;

/// A media access control (MAC) address
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MACAddress {
    octets: [u8; 6],
}

/// An error occured while parsing a MAC address
pub struct InvalidMACAddress;

impl MACAddress {
    /// The number of bits in a MAC address
    pub const BITS: usize = 6 * 8;

    /// The broadcast MAC address
    pub const BROADCAST: MACAddress = MACAddress::new([0xFF; 6]);

    /// An unspecified MAC address representation
    pub const UNSPECIFIED: MACAddress = MACAddress::new([0; 6]);

    /// Creates a new [`MACAddress`]
    ///
    /// ## Parameters
    ///  * `octets` - The octets which make up the address
    ///
    /// ## Return Value
    /// Returns the newly created [`MACAddress`]
    pub const fn new(octets: [u8; 6]) -> Self {
        MACAddress { octets }
    }

    /// Gets the octets of this MAC address
    ///
    /// ## Return Value
    /// Returns the 6 octets that make up this MAC address
    pub const fn octets(&self) -> [u8; 6] {
        self.octets
    }

    /// Gets the octets of this MAC address a slice
    pub const fn as_slice(&self) -> &[u8] {
        &self.octets
    }
}

impl From<[u8; 6]> for MACAddress {
    fn from(octets: [u8; 6]) -> Self {
        MACAddress::new(octets)
    }
}

impl FromStr for MACAddress {
    type Err = InvalidMACAddress;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let mut octets = [0; 6];
        for i in 0..6 {
            // Parse MAC segment
            let c = chars.next().ok_or(InvalidMACAddress)?;
            let mut octet = c.to_digit(16).ok_or(InvalidMACAddress)? as u8;

            octet <<= 4;

            let c = chars.next().ok_or(InvalidMACAddress)?;
            octet |= c.to_digit(16).ok_or(InvalidMACAddress)? as u8;

            octets[i] = octet;

            if i < 5 {
                // Parse colon
                let c = chars.next().ok_or(InvalidMACAddress)?;
                if c != ':' {
                    return Err(InvalidMACAddress);
                }
            }
        }

        if chars.next().is_some() {
            Err(InvalidMACAddress)
        } else {
            Ok(MACAddress::new(octets))
        }
    }
}

impl std::fmt::Display for MACAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
            self.octets[4],
            self.octets[5],
        )
    }
}

impl std::fmt::Debug for MACAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for InvalidMACAddress {}

impl std::fmt::Display for InvalidMACAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid MAC address")
    }
}

impl std::fmt::Debug for InvalidMACAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
