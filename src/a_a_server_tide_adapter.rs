mod a_a_server_builder_tide_adapter;
mod msg_queue;
mod recipient;
#[cfg(test)]
mod unit_tests;

use crate::{
    server::{Event, Mode, Server, ServerBuilder},
    Result,
};
use a_a_server_builder_tide_adapter::AAServerBuilderTideAdapter;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub use {
    msg_queue::{Msg, MsgQueue},
    recipient::Recipient,
};

/// `Server` is the type representing the simulated web a_a_server_tide_adapter.  Each thread it is invoked with will
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
pub struct AAServerTideAdapter {
    mode: Mode,
    msg_q: MsgQueue,
    termination_condition: Option<Event>,
    threads: usize,
}

impl AAServerTideAdapter {
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

    // Predicate which determines whether the a_a_server_tide_adapter should shut itself down or not.
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
impl Server for AAServerTideAdapter {
    /// Constructor which yields a `ServerBuilder` (not a `Server`).  This is an implementation of
    /// the builder pattern.
    #[allow(clippy::new_ret_no_self, clippy::must_use_candidate)]
    fn new() -> AAServerBuilderTideAdapter {
        AAServerBuilderTideAdapter::default()
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

    /// Returns the number of threads the a_a_server_tide_adapter is using to process requests.
    fn threads(&self) -> usize {
        self.threads
    }
}

/// `Server` constructor.  It is only possible to construct a `Server` from an instance of
/// `ServerBuilder`.  This ensures that any `Server` instance which exists is a valid configuration
/// and is effectively immutable.
/// This constructor contains policy:
///     - If `ServerBuilder::mode is unspecified, this constructor will set `Server`'s configuration
///       to `Mode::Asynchronous`.
///     - If `ServerBuilder::threads` is unspecified, this constructor will set Server`'s
///       configuration to half the CPU's logical cores or 1 thread (whichever value is greater).
impl From<AAServerBuilderTideAdapter> for AAServerTideAdapter {
    fn from(builder: AAServerBuilderTideAdapter) -> Self {
        Self {
            mode: builder.mode.unwrap_or(Mode::Asynchronous),
            msg_q: builder.msg_q,
            termination_condition: builder.termination_condition,
            threads: builder
                .threads
                .unwrap_or(usize::max(num_cpus::get() / 2, 1)),
        }
    }
}

/// Provides a definition for equality of two `Server` instances.  Does not compare message queue
/// state (which is driven by external clients).  Useful primarily for tests.
impl PartialEq for AAServerTideAdapter {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_condition == rhs.termination_condition
            && self.threads == rhs.threads
    }
}
