mod event;
mod mode;
mod server_builder;
#[cfg(test)]
mod unit_tests;

use crate::{Result, consts::*};
use std::{
    thread::{sleep},
    os::unix::thread::{JoinHandleExt, RawPthread},
    thread
};
use tide;
pub use {event::Event, mode::Mode, server_builder::ServerBuilder};
use std::fmt::{Debug, Formatter, Error as StdFmtError};
use std::ops::Deref;

struct TideApp(tide::App<()>);

impl Debug for TideApp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), StdFmtError> {
        write!(f, "tide::App<()>")
    }
}

impl Deref for TideApp {
    type Target = tide::App<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for TideApp {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq)]
pub struct Server {
    app: TideApp,
    mode: Mode,
    threads: usize,
}

impl Server {
    #[inline]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    fn configure_server(&mut self) -> &mut Self {
        self.app.at(ROUTE_GET_INFO)
            .get(|_| async move { format!("Hello, {}-bit world!", 0_usize.count_zeros()) });
        self
    }

    fn launch(&self) -> RawPthread {
        let jh = thread::spawn(move || self.app.serve(SERVER_ADDR));
        jh.into_pthread_t()
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn run_until(mut self, event: Event) -> Result<()> {
        let server = self.configure_server().launch();
        match event {
            Event::TimeElapsed(dur) => sleep(dur),
        };

        Ok(())
    }

    #[inline]
    pub const fn threads(&self) -> usize {
        self.threads
    }
}

impl From<ServerBuilder> for Server {
    fn from(builder: ServerBuilder) -> Self {
        Self {
            app: TideApp(tide::App::new()),
            mode: builder.mode.unwrap_or_else(|| Mode::Asynchronous),
            threads: builder.threads.unwrap_or_else(|| 1),
        }
    }
}

