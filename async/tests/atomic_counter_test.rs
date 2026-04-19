use r#async::medium::atomic_counter::atomic_counter;

#[test]
fn test_final_value() {
    assert_eq!(atomic_counter(), 1000);
}

#[test]
fn test_consistency() {
    for _ in 0..10 {
        assert_eq!(atomic_counter(), 1000);
    }
}
