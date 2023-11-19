//! IP address utilities

pub mod v4;
pub mod v6;

mod cidr;
mod error;

pub use cidr::IPCIDR;
pub use error::{InvalidCIDRError, InvalidPrefixError};

pub use std::net::IpAddr as IPAddress;
pub use std::net::SocketAddr as IPSocketAddress;
