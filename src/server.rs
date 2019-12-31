mod bind_network_interface;
mod server_builder;
mod terminate_condition;
#[cfg(test)]
mod unit_tests;

use crate::{consts::msg, Result};
pub use bind_network_interface::BindNetworkInterface;
pub use server_builder::ServerBuilder;
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr},
    thread::{self, JoinHandle},
    time::Instant,
};
use terminate_condition::TerminateCondition;

#[derive(Debug)]
pub struct Server<'a> {
    bind_network_interface: BindNetworkInterface,
    ip_addr: Option<IpAddr>,
    join_handle: Option<JoinHandle<Result<()>>>,
    started: bool,
    terminate_condition: TerminateCondition,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Server<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn block(&self) -> Option<Result<()>> {
        self.join_handle.and_then(|jh| jh.join()).or(None)
    }

    fn set_ip_addr(&mut self) -> Result<&mut Self> {
        self.ip_addr = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
        Ok(self)
    }

    pub fn start(&mut self) -> Result<IpAddr> {
        // atomic CAS not required because this entire method is single threaded (`&mut self`)
        if !self.started {
            self.started = true;
            self.set_ip_addr()?;
            let terminate_condition = self.terminate_condition;
            self.join_handle =
                thread::spawn(move || -> Result<()> { Self::process_msgs(terminate_condition) });
        }

        Ok(self
            .ip_addr
            .unwrap_or_else(|| unreachable!(msg::ERR_INTERNAL_IP_ADDR_NOT_SET)))
    }

    #[inline]
    fn process_next_req() -> Result<()> {
        Ok(())
    }

    fn process_msgs(terminate_condition: TerminateCondition) -> Result<()> {
        match terminate_condition {
            TerminateCondition::Never => loop {
                Self::process_next_req()?;
            },

            TerminateCondition::AfterDuration(run_duration_limit) => {
                let start_instant = Instant::now();

                while Instant::now() - start_instant < run_duration_limit {
                    Self::process_next_req()?;
                }
            }
        };
        Ok(())
    }
}
