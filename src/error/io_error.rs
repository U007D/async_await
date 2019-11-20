use derive_more::From;
use std::io::Error;

#[derive(Debug, From)]
pub struct IoError(Error);

impl PartialEq for IoError {
    fn eq(&self, other: &Self) -> bool {
        self.0.kind() == other.0.kind()
    }
}
