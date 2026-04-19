use r#async::easy::oneshot_channel::oneshot_demo;

#[tokio::test]
async fn test_oneshot() {
    assert_eq!(oneshot_demo().await, "done");
}
