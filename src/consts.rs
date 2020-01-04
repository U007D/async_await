use std::net::{IpAddr, Ipv4Addr};

pub mod msg;

pub const NANOS_PER_SEC: u32 = 1_000_000_000;
pub const ROUTE_GET_INFO: &str = "/get_info";
pub const DEFAULT_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
/// Port 0 indicates the OS is to assign a port number to a `TcpListener::bind()` (server-side
/// websocket creation).
pub const DEFAULT_PORT: u16 = 0;
