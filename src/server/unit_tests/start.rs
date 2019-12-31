use super::*;

#[test]
fn start__returns_expected_ip_address() -> Result<()> {
    // given a server instance configured to bind to the localhost network interface
    let mut sut = Server::new()
        .bind_network_interface(BindNetworkInterface::LocalHost)
        .build();

    // when the server is started
    let res = sut.start()?;

    // then the ip address returned should be the localhost address
    assert_ge!(res, IpAddr::V4(Ipv4Addr::LOCALHOST));
    Ok(())
}
