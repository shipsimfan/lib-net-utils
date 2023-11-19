//! IPv4 address utilities

mod cidr;

pub use cidr::IPv4CIDR;

pub use std::net::Ipv4Addr as IPv4Address;
pub use std::net::SocketAddrV4 as IPv4SocketAddress;
