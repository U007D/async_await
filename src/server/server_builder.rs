#[cfg(test)]
mod unit_tests;

use super::*;
use crate::consts::{DEFAULT_IP_ADDR, DEFAULT_PORT};

#[derive(Debug, Default, PartialEq)]
pub struct ServerBuilder {
    ip_addr: Option<IpAddr>,
    port: Option<u16>,
    term_cond: Option<Terminate>,
}

impl ServerBuilder {
    pub fn build(self) -> Server {
        Server {
            ip_addr: self.ip_addr.unwrap_or(DEFAULT_IP_ADDR),
            tcp_listener: None,
            port: self.port.unwrap_or(DEFAULT_PORT),
            term_cond: self.term_cond.unwrap_or(Terminate::Never),
        }
    }

    pub fn ip_address(mut self, ip_addr: IpAddr) -> Self {
        self.ip_addr = Some(ip_addr);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn terminate_condition(mut self, terminate_condition: Terminate) -> Self {
        self.term_cond = Some(terminate_condition);
        self
    }
}
