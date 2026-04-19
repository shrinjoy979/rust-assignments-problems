use r#async::hard::thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn test_pool_execution() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }

    // Give some time for workers to complete
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(*counter.lock().unwrap(), 8);
}

#[test]
fn test_pool_size() {
    let pool = ThreadPool::new(3);
    assert_eq!(pool.workers.len(), 3);
}
