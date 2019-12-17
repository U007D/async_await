mod msg_queue;
mod recipient;
mod server_builder_tide;
#[cfg(test)]
mod unit_tests;

use crate::{ports::server::prelude::*, Result};
use server_builder_tide::ServerBuilderTide;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub use {
    msg_queue::{Msg, MsgQueue},
    recipient::Recipient,
};

/// `Server` is the type representing the simulated web `server_tide`.  Each thread it is invoked
/// with will read from a single shared queue, either synchronously or asynchronously, depending on
/// how the user has configured it.  It will measure statistics on throughput, latency, queue depth,
/// to clearly illustrate performance in is various configurations.
///
/// Because of the number of configuration permutations possible, `Server` is constructed using the
/// [builder pattern](https://en.wikipedia.org/wiki/Builder_pattern).  Specifically, `Server`'s
/// constructor creates a (mutable) `ServerBuilder`, which the user configures with various options.
/// Once complete, the user calls `ServerBuilder::build()` to construct the actual `Server`
/// instance, which is not mutable.  This ensures it is not possible to construct a `Server` with
/// an illegal configuration; any attempts to do so will be met with a compile-time error.
#[derive(Debug)]
pub struct ServerTide {
    mode: Mode,
    msg_q: MsgQueue,
    termination_condition: Option<Event>,
    threads: usize,
}

impl ServerTide {
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

    // Predicate which determines whether the server_tide should shut itself down or not.
    fn should_terminate(&self) -> bool {
        self.termination_condition
            .map_or(false, |event| match event {
                Event::TimeElapsed(deadline) => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .ok()
                    .map_or(true, |now| deadline.checked_sub(now).is_none()),
            })
    }
}
impl Server for ServerTide {
    type ServerBuilder = ServerBuilderTide;

    /// Constructor which yields a `ServerBuilder` (not a `Server`).  This is an implementation of
    /// the builder pattern.
    #[allow(clippy::new_ret_no_self, clippy::must_use_candidate)]
    fn new() -> Self::ServerBuilder {
        Self::ServerBuilder::default()
    }

    /// Returns the `Mode` of the `Server`.
    fn mode(&self) -> &Mode {
        &self.mode
    }

    /// Launches the `Server`.
    fn serve(mut self) -> Result<()> {
        self.init_server()?;
        while !self.should_terminate() {}
        Ok(())
    }

    /// Returns the number of threads the server_tide is using to process requests.
    fn threads(&self) -> usize {
        self.threads
    }
}

/// Provides a definition for equality of two `Server` instances.  Does not compare message queue
/// state (which is driven by external clients).  Useful primarily for tests.
impl PartialEq for ServerTide {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_condition == rhs.termination_condition
            && self.threads == rhs.threads
    }
}
