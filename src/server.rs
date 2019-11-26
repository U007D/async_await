mod event;
mod mode;
mod msg_queue;
mod recipient;
mod server_builder;
#[cfg(test)]
mod unit_tests;

use crate::Result;
use std::time::Instant;
pub use {
    event::Event,
    mode::Mode,
    msg_queue::{Msg, MsgQueue},
    recipient::Recipient,
    server_builder::ServerBuilder,
};

#[derive(Debug)]
pub struct Server {
    event_status: Option<Event>,
    mode: Mode,
    msg_q: MsgQueue,
    termination_event: Option<Event>,
    threads: usize,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    fn init_server(&mut self) -> &mut Self {
        self.termination_event.map(|ev| match ev {
            Event::TimeElapsed(_) => self.event_status = Some(Event::TimeElapsed(Instant::now())),
        });
        self
    }

    fn met_terminate_condition(&self) -> bool {}

    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn run_until(mut self) -> Result<()> {
        self.init_server().serve();
        Ok(())
    }

    fn serve(&self) -> &Self {
        met_terminate_condition.or_else(|| unimplemented!());
        self
    }

    #[inline]
    pub const fn threads(&self) -> usize {
        self.threads
    }
}

impl From<ServerBuilder> for Server {
    fn from(builder: ServerBuilder) -> Self {
        Self {
            event_status: None,
            mode: builder.mode.unwrap_or_else(|| Mode::Asynchronous),
            msg_q: builder.msg_q,
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
