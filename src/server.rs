mod event;
mod mode;
mod msg_queue;
mod recipient;
mod server_builder;
#[cfg(test)]
mod unit_tests;

use crate::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub use {
    event::Event,
    mode::Mode,
    msg_queue::{Msg, MsgQueue},
    recipient::Recipient,
    server_builder::ServerBuilder,
};

#[derive(Debug)]
pub struct Server {
    mode: Mode,
    msg_q: MsgQueue,
    termination_condition: Option<Event>,
    threads: usize,
}

impl Server {
    #[allow(clippy::new_ret_no_self, clippy::must_use_candidate)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn serve(mut self) -> Result<()> {
        self.init_server()?;
        while !self.should_terminate() {}
        Ok(())
    }

    fn init_server(&mut self) -> Result<&mut Self> {
        // Convert TimeElapsed(Duration) from relative to absolute value of now + the relative
        // duration.
        if let Some(event) = self.termination_condition {
            let deadline = match event {
                Event::TimeElapsed(duration) => {
                    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| {
                        Event::TimeElapsed(
                            d.checked_add(duration)
                                .map_or_else(Duration::default, |d| d),
                        )
                    })
                }
            };
            self.termination_condition = Some(deadline?);
        }
        Ok(self)
    }

    fn should_terminate(&self) -> bool {
        self.termination_condition
            .map_or(false, |event| match event {
                Event::TimeElapsed(deadline) => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .ok()
                    .map_or(true, |now| deadline.checked_sub(now).is_none()),
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
            mode: builder.mode.unwrap_or_else(|| Mode::Asynchronous),
            msg_q: builder.msg_q,
            termination_condition: builder.termination_condition,
            threads: builder.threads.unwrap_or_else(|| 1),
        }
    }
}

impl PartialEq for Server {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_condition == rhs.termination_condition
            && self.threads == rhs.threads
    }
}
