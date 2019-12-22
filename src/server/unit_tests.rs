use super::*;

use super::*;
use more_asserts::*;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

#[test]
fn start__respects_after_duration_value() -> Result<()> {
    // given a server instance configured with a specific delay before exiting
    let expected_delay = Duration::from_millis(1);
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(expected_delay))
        .build();

    // when started
    let start = Instant::now();
    let _res = sut.start()?;
    sut.block();
    let elapsed_time = Instant::now() - start;

    // then the server should respected the specified delay
    assert_ge!(elapsed_time, expected_delay);
    Ok(())
}

#[test]
fn start__returns_expected_ip_address() -> Result<()> {
    // given a server instance configured to bind to the localhost network interface
    let expected_delay = Duration::from_millis(1);
    let mut sut = Server::new()
        .bind_network_interface(BindNetworkInterface::LocalHost)
        .build();

    // when started
    let res = sut.start()?;

    // then the ip address returned should be the localhost address
    assert_ge!(res, IpAddr::V4(Ipv4Addr::LOCALHOST));
    Ok(())
}
