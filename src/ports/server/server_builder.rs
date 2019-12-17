use super::{Event, Mode, Server};

pub trait ServerBuilder: Default + PartialEq {
    type Server: Server;

    fn build(self) -> Self::Server;
    fn mode(self, mode: Mode) -> Self;
    fn termination_condition(self, event: Event) -> Self;
    fn threads(self, threads: usize) -> Self;
}
