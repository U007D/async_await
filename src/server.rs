mod server_builder;
mod terminate_condition;
#[cfg(test)]
mod unit_tests;

use crate::Result;
use async_std::{
    io,
    net::{IpAddr, SocketAddr, TcpListener},
};
use futures::executor::block_on;
pub use server_builder::ServerBuilder;
use terminate_condition::TerminateCondition;

#[derive(Debug)]
pub struct Server {
    ip_addr: IpAddr,
    tcp_listener: Option<TcpListener>,
    port: u16,
    terminate_condition: TerminateCondition,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    async fn init_tcp_listener(&self) -> Result<TcpListener> {
        Ok(TcpListener::bind(SocketAddr::new(self.ip_addr, self.port)).await?)
    }
    #[inline]
    async fn process_msgs() -> Result<()> {
        Ok(())
    }

    async fn run(&self) -> Result<()> {
        match self.terminate_condition {
            TerminateCondition::Never => loop {
                Self::process_msgs().await?
            },
            TerminateCondition::AfterDuration(limit) => {
                io::timeout(limit, Self::process_msgs().await?)
                    .await
                    .unwrap_or_else(|_| Ok(()))?
            }
        }
    }

    fn start(&mut self) -> Result<()> {
        block_on(async {
            self.tcp_listener = Some(self.init_tcp_listener().await?);
            self.run().await?;
            Ok(())
        })
    }
}
