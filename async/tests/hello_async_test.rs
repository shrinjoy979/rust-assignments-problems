use r#async::easy::hello_async::hello_async;

#[tokio::test]
async fn test_hello() {
    assert_eq!(hello_async().await, "hello from tokio");
}
