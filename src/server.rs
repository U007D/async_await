mod event;
mod mode;
mod server_builder;
use crate::Result;
pub use {event::Event, mode::Mode, server_builder::ServerBuilder};

pub trait Server: PartialEq {
    fn new<SB: ServerBuilder>() -> SB;
    fn mode(&self) -> &Mode;
    fn serve(self) -> Result<()>;
    fn threads(&self) -> usize;
}
