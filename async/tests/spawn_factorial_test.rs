use r#async::easy::spawn_factorial::spawn_factorial;

#[tokio::test]
async fn test_factorial() {
    assert_eq!(spawn_factorial().await, 120);
}
