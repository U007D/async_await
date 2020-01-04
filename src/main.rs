//#![feature(result_map_or_else)]
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
// Uncomment before ship to find redundant crates, debug remnants, missing license files & more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod consts;
mod error;
mod server;

use error::Error;
use pico_args::Arguments;
use tokio;

/// Convenience alias for the `Result` type encoding `error::Error` as the default error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    // see examples/pico_args.rs for argument parsing example
    let _args_parser = Arguments::from_env();

    Ok(())
}
