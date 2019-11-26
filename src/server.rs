mod event;
mod mode;
mod msg_queue;
mod recipient;
mod server_builder;
#[cfg(test)]
mod unit_tests;

use crate::Result;
pub use {
    event::Event,
    mode::Mode,
    msg_queue::{Msg, MsgQueue},
    recipient::Recipient,
    server_builder::ServerBuilder,
};

#[derive(Debug)]
pub struct Server {
    msg_q: MsgQueue,
    mode: Mode,
    termination_event: Option<Event>,
    threads: usize,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    fn init_server(&self) -> &Self {
        unimplemented!()
    }

    #[inline]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn run_until(mut self, _event: Event) -> Result<()> {
        self.init_server().serve();
        Ok(())
    }

    fn serve(&self) -> &Self {
        unimplemented!()
    }

    #[inline]
    pub const fn threads(&self) -> usize {
        self.threads
    }
}

impl From<ServerBuilder> for Server {
    fn from(builder: ServerBuilder) -> Self {
        Self {
            msg_q: builder.msg_q,
            mode: builder.mode.unwrap_or_else(|| Mode::Asynchronous),
            termination_event: builder.termination_event,
            threads: builder.threads.unwrap_or_else(|| 1),
        }
    }
}

impl PartialEq for Server {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_event == rhs.termination_event
            && self.threads == rhs.threads
    }
}
