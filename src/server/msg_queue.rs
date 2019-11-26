mod msg;
use crate::MpmcQueue;
use crossbeam_deque::{Injector, Steal};
pub use msg::Msg;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct MsgQueue {
    queue: Injector<Msg>,
    len: AtomicUsize,
}

impl MsgQueue {
    pub fn new() -> Self {
        Self {
            queue: Injector::new(),
            len: AtomicUsize::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }
}

impl MpmcQueue for MsgQueue {
    type Item = Msg;

    fn pop(&self) -> Option<Self::Item> {
        loop {
            match self.queue.steal() {
                Steal::Empty => break None,
                Steal::Retry => (),
                Steal::Success(item) => {
                    let prev_value = self.len.fetch_sub(1, Ordering::Release);
                    debug_assert_ne!(prev_value, 0);

                    break Some(item);
                }
            }
        }
    }

    fn push(&self, msg: Msg) -> &Self {
        let prev_val = self.len.fetch_add(1, Ordering::Release);
        debug_assert_ne!(prev_val, usize::max_value());

        self.queue.push(msg);
        &self
    }
}
