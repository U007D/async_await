mod server_builder;
mod terminate_condition;
#[cfg(test)]
mod unit_tests;

use crate::{Error, Result};
use async_std::{
    future::{self, Future},
    net::{IpAddr, SocketAddr, TcpListener},
    prelude::FutureExt,
};
use futures_executor::block_on;
pub use server_builder::ServerBuilder;
use std::net::{SocketAddr, TcpListener};
use std::time::{Duration, Instant};
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

    async fn process_msgs() -> Result<()> {
        Ok(())
    }

    fn run_duration(&self) -> Option<Duration> {
        match self.terminate_condition {
            TerminateCondition::Never => None,
            TerminateCondition::AfterDuration(duration) => Some(duration),
        }
    }

    async fn time_limit_elapsed(duration: Option<Duration>) -> Result<()> {
        let start = Instant::now();
        loop {
            duration
                .map_or_else(
                    || future::pending(),
                    |time_limit| {
                        let elapsed_time = Instant::now() - start;
                        match elapsed_time < time_limit {
                            true => future::pending::<Result<()>>(),
                            false => future::ready(Ok(())),
                        }
                    },
                )
                .await;
        }
    }

    fn start(&mut self) -> Result<()> {
        block_on(async {
            self.tcp_listener = {
                socket_addr = SocketAddr::new(self.ip_addr, self.port);
                Some(TcpListener::bind(socket_addr)).await?
            };

            let mut fut_builder = Self::process_msgs();

            match self.terminate_condition {
                TerminateCondition::Never => (),
                TerminateCondition::AfterDuration(dur) => fut_builder = fut_builder.timeout(dur),
            }
            let fut = fut_builder;

            fut.await.map_or_else(
                |err| match err {
                    Error::Timeout(_) => Ok(()),
                    e => Err(e),
                },
                |ok| Ok(ok),
            );
            Ok(res?)
        })
    }
}
