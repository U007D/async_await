use std::time::Duration;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Event {
    TimeElapsed(Duration),
}
