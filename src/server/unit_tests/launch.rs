use super::*;
use more_asserts::*;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

#[test]
fn launch__respects_after_duration_value() -> Result<()> {
    // given
    let expected_delay = Duration::from_millis(1);
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(expected_delay))
        .build();

    // when
    let start = Instant::now();
    let _res = sut.start()?;
    let elapsed_time = Instant::now() - start;

    // then
    assert_ge!(elapsed_time, expected_delay);
    Ok(())
}

#[test]
fn start__returns_expected_ip_address() -> Result<()> {
    // given
    let expected_delay = Duration::from_millis(1);
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(expected_delay))
        .bind_network_interface(BindNetworkInterface::LocalHost)
        .build();

    // when
    let start = Instant::now();
    let res = sut.start()?;
    let elapsed_time = Instant::now() - start;

    // then
    assert_ge!(res, IpAddr::V4(Ipv4Addr::LOCALHOST));
    Ok(())
}
