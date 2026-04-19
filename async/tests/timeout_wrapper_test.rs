use r#async::medium::timeout_wrapper::timeout_wrapper;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_success() {
    let f = async { sleep(Duration::from_millis(10)).await; 42 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(50)).await, Ok(42));
}

#[tokio::test]
async fn test_timeout() {
    let f = async { sleep(Duration::from_millis(100)).await; 42 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(10)).await, Err("Timeout".to_string()));
}
