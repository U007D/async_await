use super::*;
use std::time::Duration;

#[test]
fn build__respects_bind_network_interface() {
    // setup
    let expected_bind_network_interface = BindNetworkInterface::Any;

    // given a `ServerBuilder` configured with a particular `BindNetworkInterface` mode
    let sut = Server::new().bind_network_interface(expected_bind_network_interface);

    // when constructed
    let res = sut.build();

    // then the resulting server should be configured as expected
    assert_eq!(res.bind_network_interface, expected_bind_network_interface);
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
