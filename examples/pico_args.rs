#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod app_args {
    #[derive(Clone, Debug, PartialEq)]
    pub struct AppArgs {
        pub help: bool,
        pub number: u32,
        pub opt_number: Option<u32>,
        pub width: u32,
        pub free: Vec<String>,
    }
}
mod consts {
    pub mod msg {
        #![allow(dead_code)]

        pub const ERR_ARG_NOT_VALID_USIZE: &str =
            "Error: supplied argument could not be converted to a `usize`";
        pub const ERR_ARG_NOT_POSITIVE: &str =
            "Error: supplied argument must be positive (ie. > 0)";
        pub const ERR_ARG_PROCESSING: &str = "Error processing command-line argument";
    }
}

mod error {
    #![allow(clippy::pub_enum_variant_names)]
    use crate::consts::msg;
    use derive_more::Display;

    #[derive(Debug, Display)]
    pub enum Error {
        #[display(fmt = "{}: {:?}", msg::ERR_ARG_NOT_VALID_USIZE, source)]
        ArgInvalidIntegralValue { source: std::num::ParseIntError },
        #[display(fmt = "{}: {}", msg::ERR_ARG_NOT_POSITIVE, value)]
        ArgNonPositiveValue { value: String },
        #[display(fmt = "{}: {:?}", msg::ERR_ARG_PROCESSING, source)]
        ArgProcessing { source: pico_args::Error },
    }

    impl From<pico_args::Error> for Error {
        fn from(source: pico_args::Error) -> Self {
            Self::ArgProcessing { source }
        }
    }
}

use pico_args::Arguments;
use {app_args::AppArgs, error::Error};

pub type Result<T, E = Error> = std::result::Result<T, E>;

fn parse_width(s: &str) -> Result<u32> {
    s.parse()
        .map_err(|e| Error::ArgInvalidIntegralValue { source: e })
        .and_then(|w| match w > 0 {
            true => Ok(w),
            false => Err(Error::ArgNonPositiveValue {
                value: s.to_string(),
            }),
        })
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    println!(
        "{:?}",
        AppArgs {
            help: args.contains(["-h", "--help"]),
            number: args.opt_value_from_str("--number")?.unwrap_or(5),
            opt_number: args.opt_value_from_str("--opt-number")?,
            width: args
                .opt_value_from_fn("--width", parse_width)?
                .unwrap_or(10),
            free: args.free()?,
        }
    );

    Ok(())
}
