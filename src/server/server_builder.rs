use super::{Mode, Server};

#[allow(clippy::module_name_repetitions)]
pub struct ServerBuilder {
    pub(super) mode: Option<Mode>,
    pub(super) threads: Option<usize>,
}

impl ServerBuilder {
    pub fn build(self) -> Server {
        Server::from(self)
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = Some(threads);
        self
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            mode: None,
            threads: None,
        }
    }
}
