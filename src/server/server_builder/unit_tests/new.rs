use super::*;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn new__sets_server_to_expected_defaults() {
    // setup
    let expected_ip_addr = IpAddr::from(Ipv4Addr::LOCALHOST);
    let expected_terminate_condition = Terminate::Never;

    // given a `ServerBuilder` configured with default settings
    let sut = Server::new();
    // when constructed
    let res = sut.build();

    // then the resulting server should be configured as expected
    assert_eq!(res.ip_addr, expected_ip_addr);
    assert_eq!(res.term_cond, expected_terminate_condition);
}
