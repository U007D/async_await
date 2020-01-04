use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Terminate {
    Never,
    AfterDuration(Duration),
}
