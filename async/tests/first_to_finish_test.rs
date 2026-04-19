use r#async::medium::first_to_finish::first_to_finish;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_first_wins() {
    let f1 = async { sleep(Duration::from_millis(50)).await; 1 };
    let f2 = async { sleep(Duration::from_millis(10)).await; 2 };
    // Box and pin them to satisfy Unpin requirement for select!
    let f1 = Box::pin(f1);
    let f2 = Box::pin(f2);
    assert_eq!(first_to_finish(f1, f2).await, 2);
}

#[tokio::test]
async fn test_other_wins() {
    let f1 = async { sleep(Duration::from_millis(5)).await; 10 };
    let f2 = async { sleep(Duration::from_millis(100)).await; 20 };
    let f1 = Box::pin(f1);
    let f2 = Box::pin(f2);
    assert_eq!(first_to_finish(f1, f2).await, 10);
}
