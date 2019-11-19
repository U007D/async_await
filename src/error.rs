use crate::consts::msg;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, value)]
    ArgNotConvertibleToUtf8 { value: std::ffi::OsString },
}
