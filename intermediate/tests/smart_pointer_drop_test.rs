use intermediate::medium::smart_pointer_drop::Logger;
use std::sync::{Arc, Mutex};

#[test]
fn test_drop() {
    let counter = Arc::new(Mutex::new(0));
    {
        let _l = Logger {
            name: "test".to_string(),
            drop_count: Arc::clone(&counter),
        };
        assert_eq!(*counter.lock().unwrap(), 0);
    }
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_multiple_drops() {
    let counter = Arc::new(Mutex::new(0));
    {
        let _l1 = Logger { name: "1".to_string(), drop_count: Arc::clone(&counter) };
        let _l2 = Logger { name: "2".to_string(), drop_count: Arc::clone(&counter) };
    }
    assert_eq!(*counter.lock().unwrap(), 2);
}
