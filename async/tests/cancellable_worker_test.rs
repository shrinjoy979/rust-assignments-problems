use r#async::medium::cancellable_worker::cancellable_worker;

#[test]
fn test_termination() {
    // If the worker doesn't terminate, this test will hang (and eventually time out)
    cancellable_worker();
}

#[test]
fn test_runs_at_least_once() {
    // This is more of a smoke test to ensure no panics
    cancellable_worker();
}
