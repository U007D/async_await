use super::*;

#[test]
fn build__using_threads() {
    // given a `ServerBuilder` with `threads` explicitly set
    let threads = 42;
    let sut = ServerTide::new().threads(threads);

    // when built
    let res = sut.build();

    // then the `Server` should be configured as expected
    assert_eq!(
        res,
        ServerTide {
            msg_q: MsgQueue::new(),
            mode: Mode::Asynchronous,
            termination_condition: None,
            threads
        }
    )
}

#[test]
fn build__using_sync_mode() {
    // given a `ServerBuilder` with `mode` explicitly set to `Mode::Synchronous`
    let mode = Mode::Synchronous;
    let sut = ServerTide::new().mode(mode);

    // when built
    let res = sut.build();

    // then the `Server` should be configured as expected
    assert_eq!(
        res,
        ServerTide {
            msg_q: MsgQueue::new(),
            mode,
            termination_condition: None,
            threads: 1
        }
    )
}

#[test]
fn build__using_async_mode() {
    // given a `ServerBuilder` with `mode` explicitly set to `Mode::Asynchronous`
    let mode = Mode::Asynchronous;
    let expected_threads = usize::max(num_cpus::get() / 2, 1);
    let sut = ServerTide::new().mode(mode);

    // when built
    let res = sut.build();

    // then the `Server` should be configured as expected
    assert_eq!(
        res,
        ServerTide {
            msg_q: MsgQueue::new(),
            mode,
            termination_condition: None,
            threads: expected_threads,
        }
    )
}

#[test]
fn build__using_sync_mode_threads() {
    // given a `ServerBuilder` with sync `mode` and `threads` explicitly set
    let threads = 42;
    let mode = Mode::Synchronous;
    let sut = ServerTide::new().mode(mode).threads(threads);

    // when
    let res = sut.build();

    // then
    assert_eq!(
        res,
        ServerTide {
            msg_q: MsgQueue::new(),
            mode,
            termination_condition: None,
            threads
        }
    )
}

#[test]
fn build__using_async_mode_threads() {
    // given a `ServerBuilder` with async `mode` and `threads` explicitly set
    let threads = 42;
    let mode = Mode::Asynchronous;
    let sut = ServerTide::new().mode(mode).threads(threads);

    // when
    let res = sut.build();

    // then
    assert_eq!(
        res,
        ServerTide {
            msg_q: MsgQueue::new(),
            mode,
            termination_condition: None,
            threads
        }
    )
}
