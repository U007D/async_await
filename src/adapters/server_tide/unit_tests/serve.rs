use super::*;
use std::time::{Duration, Instant};

#[test]
fn serve__time_elapsed_termination_condition_respected() {
    // given a default Server instance
    let delay = Duration::from_millis(5);
    let sut = ServerTide::new()
        .termination_condition(Event::TimeElapsed(delay))
        .build();

    // when
    let start = Instant::now();
    let res = sut.serve();

    // then
    assert_ge!(Instant::now() - start, delay);
    assert!(res.is_ok())
}
