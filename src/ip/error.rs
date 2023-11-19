use std::{net::AddrParseError, num::ParseIntError};

/// An error while parsing a CIDR address
pub enum InvalidCIDRError {
    /// The address is missing
    MissingAddress,

    /// The prefix is missing
    MissingPrefix,

    /// The address could not be parsed
    AddressParseError(AddrParseError),

    /// The prefix could not be parsed
    PrefixParseError(ParseIntError),

    /// The prefix is invalid
    InvalidPrefix(InvalidPrefixError),

    /// There is more data beyond the prefix
    ExtraContent,
}

/// The CIDR prefix is invalid
pub struct InvalidPrefixError {
    /// The invalid prefix
    pub prefix: u8,
}

impl std::error::Error for InvalidCIDRError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InvalidCIDRError::AddressParseError(error) => Some(error),
            InvalidCIDRError::PrefixParseError(error) => Some(error),
            InvalidCIDRError::InvalidPrefix(error) => Some(error),

            InvalidCIDRError::MissingAddress
            | InvalidCIDRError::MissingPrefix
            | InvalidCIDRError::ExtraContent => None,
        }
    }
}

impl std::fmt::Display for InvalidCIDRError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidCIDRError::MissingAddress => write!(f, "missing address"),
            InvalidCIDRError::MissingPrefix => write!(f, "missing prefix"),
            InvalidCIDRError::AddressParseError(error) => write!(f, "{}", error),
            InvalidCIDRError::PrefixParseError(error) => write!(f, "invalid prefix - {}", error),
            InvalidCIDRError::InvalidPrefix(error) => write!(f, "{}", error),
            InvalidCIDRError::ExtraContent => write!(f, "data beyond prefix"),
        }
    }
}

impl std::fmt::Debug for InvalidCIDRError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for InvalidPrefixError {}

impl std::fmt::Display for InvalidPrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prefix {}", self.prefix)
    }
}

impl std::fmt::Debug for InvalidPrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
