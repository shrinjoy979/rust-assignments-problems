use r#async::hard::fan_out_fan_in::fan_out_fan_in;

#[tokio::test]
async fn test_square_sum() {
    assert_eq!(fan_out_fan_in(vec![1, 2, 3]), 14);
}

#[tokio::test]
async fn test_empty() {
    assert_eq!(fan_out_fan_in(vec![]), 0);
}

#[tokio::test]
async fn test_large() {
    let v = vec![1; 100];
    assert_eq!(fan_out_fan_in(v), 100);
}
