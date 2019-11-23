use super::*;

#[test]
fn build__using_threads() {
    // given a `ServerBuilder` with `threads` explicitly set
    let threads = 42;
    let sut = Server::new().threads(threads);

    // when built
    let res = sut.build();

    // then the `Server` should be configured as expected
    assert_eq!(
        res,
        Server {
            mode: Mode::Asynchronous,
            terminal_event: None,
            threads
        }
    )
}

#[test]
fn build__using_mode() {
    // given a `ServerBuilder` with `mode` explicitly set
    let mode = Mode::Synchronous;
    let sut = Server::new().mode(mode);

    // when built
    let res = sut.build();

    // then the `Server` should be configured as expected
    assert_eq!(
        res,
        Server {
            mode,
            terminal_event: None,
            threads: 1
        }
    )
}

#[test]
fn build__using_mode_threads() {
    // given a `ServerBuilder` with `mode` and `threads` explicitly set
    let threads = 42;
    let mode = Mode::Synchronous;
    let sut = Server::new().mode(mode).threads(threads);

    // when
    let res = sut.build();

    // then
    assert_eq!(
        res,
        Server {
            mode,
            terminal_event: None,
            threads
        }
    )
}
