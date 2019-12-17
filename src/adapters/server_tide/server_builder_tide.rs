use super::{Event, Mode, ServerBuilder, ServerTide};
use crate::adapters::server_tide::MsgQueue;

#[derive(Debug)]
pub struct ServerBuilderTide {
    pub(super) msg_q: MsgQueue,
    pub(super) mode: Option<Mode>,
    pub(super) termination_condition: Option<Event>,
    pub(super) threads: Option<usize>,
}

impl ServerBuilder for ServerBuilderTide {
    type Server = ServerTide;

    /// `Server` constructor.  It is only possible to construct a `Server` from an instance of
    /// `ServerBuilder`.  This ensures that any `Server` instance which exists is a valid configuration
    /// and is effectively immutable.
    /// This constructor contains policy:
    ///     - If `ServerBuilder::mode is unspecified, this constructor will set `Server`'s configuration
    ///       to `Mode::Asynchronous`.
    ///     - If `ServerBuilder::threads` is unspecified, this constructor will set Server`'s
    ///       configuration to half the CPU's logical cores or 1 thread (whichever value is greater)
    ///       if `Mode::Asynchronous`, or 1 if `Mode::Synchronous`.
    fn build(self) -> Self::Server {
        let mode = self.mode.unwrap_or(Mode::Asynchronous);
        let threads = match mode {
            // since `usize / 2` cannot overflow
            #[allow(clippy::integer_arithmetic)]
            Mode::Asynchronous => self
                .threads
                .unwrap_or_else(|| usize::max(num_cpus::get() / 2, 1)),
            Mode::Synchronous => self.threads.unwrap_or(1),
        };
        Self::Server {
            mode,
            msg_q: self.msg_q,
            termination_condition: self.termination_condition,
            threads,
        }
    }

    fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    fn termination_condition(mut self, event: Event) -> Self {
        self.termination_condition = Some(event);
        self
    }

    fn threads(mut self, threads: usize) -> Self {
        self.threads = Some(threads);
        self
    }
}

impl Default for ServerBuilderTide {
    fn default() -> Self {
        Self {
            msg_q: MsgQueue::new(),
            mode: None,
            termination_condition: None,
            threads: None,
        }
    }
}

impl PartialEq for ServerBuilderTide {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_condition == rhs.termination_condition
            && self.threads == rhs.threads
    }
}
