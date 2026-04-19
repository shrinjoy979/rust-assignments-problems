use r#async::easy::concurrent_compute::compute_concurrently;

async fn task1() -> i32 { 10 }
async fn task2() -> i32 { 20 }

#[tokio::test]
async fn test_sum() {
    assert_eq!(compute_concurrently(task1(), task2()).await, 30);
}

#[tokio::test]
async fn test_with_delays() {
    use tokio::time::{sleep, Duration};
    let f1 = async { sleep(Duration::from_millis(10)).await; 5 };
    let f2 = async { sleep(Duration::from_millis(5)).await; 5 };
    assert_eq!(compute_concurrently(f1, f2).await, 10);
}
