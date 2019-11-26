use super::*;
use std::time::{Duration, Instant};

#[test]
fn run_until__time_elapsed_respected() {
    // given a default Server instance
    let delay = Duration::from_millis(5);
    let sut = Server::new().build();

    // when
    let start = Instant::now();
    let res = sut.serve();

    // then
    assert_ge!(Instant::now() - start, delay);
    assert!(res.is_ok())
}
