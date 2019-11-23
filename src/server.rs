mod event;
mod mode;
mod server_builder;
#[cfg(test)]
mod unit_tests;

use crate::{consts::*, Result};
use crossbeam_channel::
use std::thread;
pub use {event::Event, mode::Mode, server_builder::ServerBuilder};

#[derive(Debug, PartialEq)]
pub struct Server {
    mode: Mode,
    threads: usize,
}

impl Server {
    #[inline]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn run_until(mut self, event: Event) -> Result<()> {
        match event {
            Event::TimeElapsed(dur) => thread::sleep(dur),
        };

        Ok(())
    }

    #[inline]
    pub const fn threads(&self) -> usize {
        self.threads
    }
}

impl From<ServerBuilder> for Server {
    fn from(builder: ServerBuilder) -> Self {
        Self {
            mode: builder.mode.unwrap_or_else(|| Mode::Asynchronous),
            threads: builder.threads.unwrap_or_else(|| 1),
        }
    }
}
