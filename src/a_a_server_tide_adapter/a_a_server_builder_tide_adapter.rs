use super::{AAServerTideAdapter, Event, Mode, Server, ServerBuilder};
use crate::a_a_server_tide_adapter::MsgQueue;

#[derive(Debug)]
pub struct AAServerBuilderTideAdapter {
    pub(super) msg_q: MsgQueue,
    pub(super) mode: Option<Mode>,
    pub(super) termination_condition: Option<Event>,
    pub(super) threads: Option<usize>,
}

impl ServerBuilder for AAServerBuilderTideAdapter {
    fn build(self) -> AAServerTideAdapter {
        self.into()
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

impl Default for AAServerBuilderTideAdapter {
    fn default() -> Self {
        Self {
            msg_q: MsgQueue::new(),
            mode: None,
            termination_condition: None,
            threads: None,
        }
    }
}

impl PartialEq for AAServerBuilderTideAdapter {
    fn eq(&self, rhs: &Self) -> bool {
        self.mode == rhs.mode
            && self.termination_condition == rhs.termination_condition
            && self.threads == rhs.threads
    }
}
