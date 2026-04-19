use r#async::medium::broadcast_message::broadcast_demo;

#[tokio::test]
async fn test_broadcast() {
    // If each of the 3 tasks receives the message (say 10), the sum should be 30
    let result = broadcast_demo().await;
    assert!(result > 0);
}

#[tokio::test]
async fn test_consistency() {
    for _ in 0..3 {
        broadcast_demo().await;
    }
}
