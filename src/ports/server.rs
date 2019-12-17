mod event;
mod mode;
pub mod prelude;
mod server_builder;
use crate::Result;
pub use prelude::*;

pub trait Server: PartialEq {
    type ServerBuilder: ServerBuilder;
    
    fn new() -> Self::ServerBuilder;
    fn mode(&self) -> &Mode;
    fn serve(self) -> Result<()>;
    fn threads(&self) -> usize;
}
