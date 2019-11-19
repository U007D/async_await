mod mode;
#[cfg(test)]
mod unit_tests;

use mode::Mode;

pub struct Server {}

impl Server {
    fn new() -> ServerBuilder {
        ServerBuilder {}
    }
}

struct ServerBuilder {
    mode: Option<Mode>,
    threads: Option<usize>,
}

impl ServerBuilder {
    fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    fn threads(mut self, threads: usize) -> Self {
        self.threads = Some(threads);
        self
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
