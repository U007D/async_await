use super::{Mode, Server};
use crate::server::{Event, MsgQueue};

#[derive(Debug)]
pub struct ServerBuilder {
    pub(super) msg_q: MsgQueue,
    pub(super) mode: Option<Mode>,
    pub(super) termination_event: Option<Event>,
    pub(super) threads: Option<usize>,
}

impl ServerBuilder {
    pub fn build(self) -> Server {
        self.into()
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn termination_event(mut self, event: Event) -> Self {
        self.termination_event = Some(event);
        self
    }

    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = Some(threads);
        self
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            msg_q: MsgQueue::new(),
            mode: None,
            termination_event: None,
            threads: None,
        }
    }
}

impl PartialEq for ServerBuilder {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_event == rhs.termination_event
            && self.threads == rhs.threads
    }
}
