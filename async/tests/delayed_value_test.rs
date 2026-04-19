use r#async::easy::delayed_value::delayed_value;
use std::time::Instant;

#[tokio::test]
async fn test_value() {
    assert_eq!(delayed_value(42).await, 42);
}

#[tokio::test]
async fn test_delay() {
    let start = Instant::now();
    delayed_value(1).await;
    assert!(start.elapsed().as_millis() >= 10);
}
