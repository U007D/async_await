use super::*;
use more_asserts::*;
use std::time::{Duration, Instant};

#[test]
fn block__respects_after_duration_value() -> Result<()> {
    // given a running server instance configured with a specific delay before exiting
    let expected_delay = Duration::from_millis(10);
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(expected_delay))
        .build();

    let start = Instant::now();
    let _res = sut.start()?;

    // when we block on the server until it terminates
    sut.block();
    let elapsed_time = Instant::now() - start;

    // then the server should have respected the specified delay
    assert_ge!(elapsed_time, expected_delay);
    Ok(())
}
