use r#async::medium::rwlock_config::update_and_read_config;

#[test]
fn test_config_value() {
    let final_val = update_and_read_config();
    assert!(!final_val.is_empty());
}

#[test]
fn test_thread_safety() {
    // Run multiple times to ensure no deadlocks or panics
    for _ in 0..5 {
        update_and_read_config();
    }
}
