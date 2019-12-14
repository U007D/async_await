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
// Uncomment b4 ship to find possibly redundant crates, debug remnants, missing license files & more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

//! # async_await
//!
//! ## Overview
//! This is a exemplar application designed to demonstrate Rust's
//! [`async/await`](https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html)
//! feature.
//!
//! There are a lot of misconceptions around `async/await`, particularly around the differences
//! between asynchrony, concurrency and parallelism by simulating a web server.  This application is
//! designed to help clarify what `async/await` is, and what it is not.
//!
//! ## How to Use
//! Invoke `async_await` specifying whether the server simulation should be single-threaded or
//! multi-threaded and independently, whether the server should be synchronous or asynchronous.
//! The application will measure key performance statistics and will export the relevant information
//! in a format that should be easy to translate into a graph.
//!
//! Type `async_await --help` or `async_await -h` on the command line for full help instructions.
mod consts;
mod error;
mod mpmc_queue;
mod server;

use pico_args::Arguments;
pub use {consts::*, error::Error, mpmc_queue::MpmcQueue, server::Server};

/// Convenience alias for the `Result` type encoding `async_await::error::Error` as the default
/// error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    // see examples/pico_args.rs for argument parsing example
    let _args_parser = Arguments::from_env();

    Ok(())
}
