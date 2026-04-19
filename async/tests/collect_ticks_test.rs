use r#async::medium::collect_ticks::collect_ticks;

#[tokio::test]
async fn test_count() {
    let result = collect_ticks().await;
    assert_eq!(result.len(), 5);
}

#[tokio::test]
async fn test_timing() {
    let result = collect_ticks().await;
    assert!(result[4] > result[0]);
}
