mod event;
mod mode;
mod msg_queue;
mod recipient;
mod server_builder;
#[cfg(test)]
mod unit_tests;

use crate::Result;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
pub use {
    event::Event,
    mode::Mode,
    msg_queue::{Msg, MsgQueue},
    recipient::Recipient,
    server_builder::ServerBuilder,
};

#[derive(Debug)]
pub struct Server {
    termination_condition: Option<Event>,
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

    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn serve(mut self) -> Result<()> {
        self.init_server();
        while !self.should_terminate() {}
        Ok(())
    }

    fn init_server(&mut self) -> &mut Self {
        // Convert the current time to a `Duration` (since `UNIX_EPOCH`).  The goal was to avoid
        // polluting the struct with status fields for every possible `Event` variant, so the
        // current status is encoded as an `Event`.  This means that for time, neither the
        // monotonically increasing `Instant` (preferred) nor (the non-monotonically increasing)
        // `SystemTime` are suitable--so `now()` is converted to a `Duration`.
        self.termination_condition = self.termination_event.map(|ev| match ev {
            Event::TimeElapsed(dur) => {
                Event::TimeElapsed(SystemTime::now().duration_since(UNIX_EPOCH).map_or_else(
                    |_| Duration::default(),
                    |now| now.checked_add(dur).unwrap_or_else(|| Duration::default()),
                ))
            }
        });
        self
    }

    fn should_terminate(&self) -> bool {
        self.termination_condition.map_or_else(true, |tc| match tc {
            Event::TimeElapsed(deadline) => match Instant::now().checked_sub(deadline) {
                Some(_) => true,
                None => false,
            },
        })
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
