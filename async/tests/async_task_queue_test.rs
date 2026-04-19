use r#async::hard::async_task_queue::TaskQueue;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_queue_execution() {
    let queue = TaskQueue::new(2);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        queue.push(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }).await;
    }

    sleep(Duration::from_millis(50)).await;
    assert_eq!(*counter.lock().unwrap(), 5);
}
