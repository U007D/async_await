use super::*;

#[test]
fn new__sets_server_to_expected_defaults() {
    // setup
    let expected_bind_network_interface = BindNetworkInterface::Any;
    let expected_terminate_condition = TerminateCondition::Never;

    // given a `ServerBuilder` configured with default settings
    let sut = Server::new();
    // when constructed
    let res = sut.build();

    // then the resulting server should be configured as expected
    assert_eq!(res.bind_network_interface, expected_bind_network_interface);
    assert_eq!(res.terminate_condition, expected_terminate_condition);
}
