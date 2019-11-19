use super::*;
use std::time::Duration;

#[test]
fn build__single_threaded_synchronous_server() {
    // given a default Server instance
    let sut = Server::threads(1).mode(Mode::Synchronous).build();

    // when
    let res = sut.run_until(Event::TimeElapsed(Duration::from_millis(1)));

    // then
    assert!(res.is_ok());
}
