use super::*;
use more_asserts::*;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

#[test]
fn new__ip_selector_localhost_binds_to_localhost_ip_address() -> Result<()> {
    // setup
    let expected_ip_addr = Ipv4Addr::LOCALHOST;
    let delay = Duration::from_nanos(1);

    // given a server configured to terminate and to bind to localhost (127.0.0.1)
    let mut sut = Server::new()
        .terminate_condition(TerminateCondition::AfterDuration(delay))
        .bind_network_interface(BindNetworkInterface::LocalHost)
        .build();

    // when launched
    let res = sut.start()?;

    // then
    assert_eq!(res, IpAddr::V4(Ipv4Addr::LOCALHOST));
    Ok(())
}
