mod bind_network_interface;
mod server_builder;
mod terminate_condition;
#[cfg(test)]
mod unit_tests;

use crate::{consts::msg, Result};
pub use bind_network_interface::BindNetworkInterface;
pub use server_builder::ServerBuilder;
use std::marker::PhantomData;
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Instant,
};
use terminate_condition::TerminateCondition;

#[derive(Debug, PartialEq)]
pub struct Server<'a> {
    bind_network_interface: BindNetworkInterface,
    ip_addr: Option<IpAddr>,
    started: bool,
    terminate_condition: TerminateCondition,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Server<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn block(&self) {}

    fn set_ip_addr(&mut self) -> Result<&mut Self> {
        self.ip_addr = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
        Ok(self)
    }

    pub fn start(&'a mut self) -> Result<IpAddr> {
        if !self.started {
            self.started = true;
            self.set_ip_addr()?;
            thread::spawn(|| -> Result<&'a Self> { self.process_msgs() });
        }

        Ok(self
            .ip_addr
            .unwrap_or_else(|| unreachable!(msg::ERR_INTERNAL_IP_ADDR_NOT_SET)))
    }

    #[inline]
    fn process_next_req(&'a mut self) -> Result<()> {
        Ok(())
    }

    fn process_msgs(&'a mut self) -> Result<&Self> {
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
