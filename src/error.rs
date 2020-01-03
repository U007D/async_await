mod io_error;

use crate::consts::msg;
use derive_more::{Display, From};
use io_error::IoError;
use std::{any::Any, convert::From, time::SystemTimeError};
use tokio;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, _0)]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[display(fmt = "{}: {:?}", msg::ERR_IO_ERROR_CAUSED_BY, _0)]
    IoError(IoError),
    #[display(
        fmt = "{}: {:?}",
        msg::ERR_SYSTEM_CLOCK_ADJUSTED_PRIOR_TO_UNIX_EPOCH,
        _0
    )]
    SystemTimeError(SystemTimeError),
    #[display(fmt = "{}: {:?}", msg::ERR_INTERNAL_SPAWNED_THREAD_NOT_JOINABLE, _0)]
    SpawnedThreadNotJoinable(Box<dyn Any + Send + 'static>),
    #[display(fmt = "{}: {:?}", msg::ERR_TIMEOUT, _0)]
    Timeout(tokio::time::Elapsed),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.into())
    }
}
