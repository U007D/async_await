#![allow(non_snake_case)]
use super::*;
use more_asserts::*;
use std::time::{Duration, Instant};

#[test]
fn start__returns_expected_value() -> Result<()> {
    // given a server instance configured to bind to the localhost network interface
    let expected_delay = Duration::from_millis(0);
    let expected_result = ();
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(expected_delay))
        .build();

    // when the server is started
    let res = sut.start()?;

    // then the ip address returned should be the localhost address
    assert_ge!(res, expected_result);
    Ok(())
}

#[test]
fn start__respects_after_duration_value() -> Result<()> {
    // given a running server instance configured with a specific delay before exiting
    let expected_delay = Duration::from_millis(1);
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(expected_delay))
        .build();

    // when we start on the server
    let start = Instant::now();
    let _res = sut.start()?;
    let elapsed_time = Instant::now() - start;

    // then the server should have respected the specified delay
    assert_ge!(elapsed_time, expected_delay);
    Ok(())
}
