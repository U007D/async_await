mod bind_network_interface;
mod server_builder;
mod terminate_condition;
#[cfg(test)]
mod unit_tests;

use crate::{consts::msg, Result};
pub use bind_network_interface::BindNetworkInterface;
pub use server_builder::ServerBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    net::{IpAddr, Ipv4Addr},
    time::Instant,
};
use terminate_condition::TerminateCondition;

#[derive(Debug, PartialEq)]
pub struct Server {
    bind_network_interface: BindNetworkInterface,
    ip_addr: Option<IpAddr>,
    started: bool,
    terminate_condition: TerminateCondition,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    fn set_ip_addr(&mut self) -> Result<&mut Self> {
        self.ip_addr = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
        Ok(self)
    }

    pub fn start(&mut self) -> Result<IpAddr> {
        if !self.started {
            self.started = true;
            self.set_ip_addr()?.process_reqs()?;
        }

        Ok(self
            .ip_addr
            .unwrap_or_else(|| unreachable!(msg::ERR_INTERNAL_IP_ADDR_NOT_SET)))
    }

    #[inline]
    fn process_next_req(&mut self) -> Result<()> {
        Ok(())
    }

    fn process_reqs(&mut self) -> Result<&mut Self> {
        match self.terminate_condition {
            TerminateCondition::Never => loop {
                self.process_next_req()?;
            },

            TerminateCondition::AfterDuration(launch_duration_limit) => {
                let launch_instant = Instant::now();

                while Instant::now() - launch_instant < launch_duration_limit {
                    self.process_next_req()?;
                }
            }
        };
        Ok(self)
    }
}
