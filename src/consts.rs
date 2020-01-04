use std::net::{IpAddr, Ipv4Addr};

pub mod msg;

pub const NANOS_PER_SEC: u32 = 1_000_000_000;
pub const ROUTE_GET_INFO: &str = "/get_info";
pub const DEFAULT_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
pub const DEFAULT_PORT: u16 = 1234;
