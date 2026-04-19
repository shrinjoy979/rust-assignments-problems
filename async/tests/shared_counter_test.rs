use r#async::medium::shared_counter::multithreaded_counter;

#[test]
fn test_final_value() {
    assert_eq!(multithreaded_counter(), 1000);
}

#[test]
fn test_consistency() {
    // Run multiple times to check for race conditions
    for _ in 0..10 {
        assert_eq!(multithreaded_counter(), 1000);
    }
}
