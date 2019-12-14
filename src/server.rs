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

/// `Server` is the type representing the simulated web server.  Each thread it is invoked with will
/// read from a single shared queue, either synchronously or asynchronously, depending on how the
/// user has configured it.  It will measure statistics on throughput, latency, queue depth, to
/// clearly illustrate performance in is various configurations.
///
/// Because of the number of configuration permutations possible, `Server` is constructed using the
/// [builder pattern](https://en.wikipedia.org/wiki/Builder_pattern).  Specifically, `Server`'s
/// constructor creates a (mutable) `ServerBuilder`, which the user configures with various options.
/// Once complete, the user calls `ServerBuilder::build()` to construct the actual `Server`
/// instance, which is not mutable.  This ensures it is not possible to construct a `Server` with
/// an illegal configuration; any attempts to do so will be met with a compile-time error.
#[derive(Debug)]
pub struct Server {
    mode: Mode,
    msg_q: MsgQueue,
    termination_condition: Option<Event>,
    threads: usize,
}

impl Server {
    /// Constructor which yields a `ServerBuilder` (not a `Server`).  This is an implementation of
    /// the builder pattern.
    #[allow(clippy::new_ret_no_self, clippy::must_use_candidate)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    /// Returns the `Mode` of the `Server`.
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    /// Launches the `Server`.
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

    // Predicate which determines whether the server should shut itself down or not.
    fn should_terminate(&self) -> bool {
        self.termination_condition
            .map_or(false, |event| match event {
                Event::TimeElapsed(deadline) => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .ok()
                    .map_or(true, |now| deadline.checked_sub(now).is_none()),
            })
    }

    /// Returns the number of threads the server is using to process requests.
    #[inline]
    pub const fn threads(&self) -> usize {
        self.threads
    }
}

/// `Server` constructor.  It is only possible to construct a `Server` from an instance of
/// `ServerBuilder`.  This ensures that any `Server` instance which exists is a valid configuration
/// and is effectively immutable.
impl From<ServerBuilder> for Server {
    fn from(builder: ServerBuilder) -> Self {
        Self {
            mode: builder.mode.unwrap_or(Mode::Asynchronous),
            msg_q: builder.msg_q,
            termination_condition: builder.termination_condition,
            threads: builder.threads.unwrap_or(1),
        }
    }
}

/// Provides a definition for equality of two `Server` instances.  Does not compare message queue
/// state (which is driven by external clients).  Useful primarily for tests.
impl PartialEq for Server {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_condition == rhs.termination_condition
            && self.threads == rhs.threads
    }
}
