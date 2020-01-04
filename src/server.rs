mod server_builder;
mod terminate_condition;
#[cfg(test)]
mod unit_tests;

use crate::{Error, Result};
pub use server_builder::ServerBuilder;
use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use terminate_condition::Terminate;
use tokio::{
    net::TcpListener,
    runtime::Runtime,
    time::{timeout, Instant},
};

#[derive(Debug)]
pub struct Server {
    ip_addr: IpAddr,
    tcp_listener: Option<TcpListener>,
    port: u16,
    term_cond: Terminate,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    async fn process_msgs() -> Result<()> {
        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        let mut tokio = Runtime::new()?;
        tokio.block_on(async {
            self.tcp_listener =
                Some(TcpListener::bind(SocketAddr::new(self.ip_addr, self.port)).await?);

            match self.term_cond {
                Terminate::Never => Self::process_msgs().await,
                Terminate::AfterDuration(dur) => timeout(dur, Self::process_msgs())
                    .await
                    .unwrap_or_else(|_| Ok(())),
            }
        })
    }
}
