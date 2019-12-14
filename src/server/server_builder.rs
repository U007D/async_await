use super::{Event, Mode, Server};

pub trait ServerBuilder: Default + PartialEq {
    fn build<S: Server>(self) -> S
    where
        Self: Sized;
    fn mode(self, mode: Mode) -> Self;
    fn termination_condition(self, event: Event) -> Self;
    fn threads(self, threads: usize) -> Self;
}
