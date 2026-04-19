use r#async::hard::manual_future::ReadyFuture;

#[tokio::test]
async fn test_ready() {
    let f = ReadyFuture { value: Some(42) };
    assert_eq!(f.await, 42);
}

#[tokio::test]
async fn test_string() {
    let f = ReadyFuture { value: Some("ready".to_string()) };
    assert_eq!(f.await, "ready");
}
