use r#async::hard::task_scheduler::TaskScheduler;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_scheduling() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));

    scheduler.start();

    {
        let counter = Arc::clone(&counter);
        scheduler.schedule(Duration::from_millis(50), move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }

    // Should not have run yet
    sleep(Duration::from_millis(10)).await;
    assert_eq!(*counter.lock().unwrap(), 0);

    // Should have run now
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[tokio::test]
async fn test_multiple_tasks() {
    let scheduler = TaskScheduler::new();
    let result = Arc::new(Mutex::new(Vec::new()));

    scheduler.start();

    {
        let result = Arc::clone(&result);
        scheduler.schedule(Duration::from_millis(100), move || {
            result.lock().unwrap().push(2);
        });
    }
    {
        let result = Arc::clone(&result);
        scheduler.schedule(Duration::from_millis(20), move || {
            result.lock().unwrap().push(1);
        });
    }

    sleep(Duration::from_millis(150)).await;
    let final_res = result.lock().unwrap();
    assert_eq!(*final_res, vec![1, 2]); // Order based on delay
}
