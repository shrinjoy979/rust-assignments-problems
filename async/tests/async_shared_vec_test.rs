use r#async::medium::async_shared_vec::async_shared_vec;

#[tokio::test]
async fn test_length() {
    assert_eq!(async_shared_vec().await, 50);
}

#[tokio::test]
async fn test_consistency() {
    for _ in 0..5 {
        assert_eq!(async_shared_vec().await, 50);
    }
}
