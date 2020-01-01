use super::*;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[test]
fn build__respects_bind_network_interface() {
    // setup
    let expected_ip_addr = IpAddr::from(Ipv4Addr::LOCALHOST);

    // given a `ServerBuilder` configured with a particular IP address
    let sut = Server::new().ip_address(expected_ip_addr);

    // when constructed
    let res = sut.build();

    // then the resulting server should be configured as expected
    assert_eq!(res.ip_addr, expected_ip_addr);
}

#[test]
fn build__respects_terminate_condition() {
    // setup
    let expected_terminate_condition = TerminateCondition::AfterDuration(Duration::from_millis(42));

    // given a `ServerBuilder` configured with a particular `TerminateCondition` mode
    let sut = Server::new().terminate_condition(expected_terminate_condition);

    // when constructed
    let res = sut.build();

    // then the resulting server should be configured as expected
    assert_eq!(res.terminate_condition, expected_terminate_condition);
}
