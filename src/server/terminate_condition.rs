use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TerminateCondition {
    Never,
    AfterDuration(Duration),
}
