mod io_error;

use crate::consts::msg;
use derive_more::{Display, From};
use io_error::IoError;
use std::convert::From;

#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, value)]
    ArgNotConvertibleToUtf8 { value: std::ffi::OsString },
    #[display(fmt = "{}: {:?}", msg::ERR_IO_ERROR_CAUSED_BY, _0)]
    IoError(IoError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.into())
    }
}
