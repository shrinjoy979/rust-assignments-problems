use r#async::medium::async_producer_consumer::async_producer_consumer;

#[tokio::test]
async fn test_count() {
    let result = async_producer_consumer().await;
    assert_eq!(result.len(), 10);
}

#[tokio::test]
async fn test_values() {
    let result = async_producer_consumer().await;
    assert!(result.contains(&1));
    assert!(result.contains(&10));
}
