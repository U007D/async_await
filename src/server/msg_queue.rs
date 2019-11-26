mod msg;
use crate::MpmcQueue;
use crossbeam_deque::{Injector, Steal};
pub use msg::Msg;

#[derive(Debug)]
pub struct MsgQueue(Injector<Msg>);

impl MsgQueue {
    pub fn new() -> Self {
        Self(Injector::new())
    }
}

impl MpmcQueue for MsgQueue {
    type Item = Msg;

    fn pop(&self) -> Option<Self::Item> {
        loop {
            match self.0.steal() {
                Steal::Empty => break None,
                Steal::Retry => (),
                Steal::Success(item) => break Some(item),
            }
        }
    }

    fn push(&self, msg: Msg) -> &Self {
        self.0.push(msg);
        &self
    }
}
