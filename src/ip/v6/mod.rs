//! IPv6 address utilities

mod cidr;

pub use cidr::IPv6CIDR;

pub use std::net::Ipv6Addr as IPv6Address;
pub use std::net::SocketAddrV6 as IPv6SocketAddress;
