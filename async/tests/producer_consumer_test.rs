use r#async::medium::producer_consumer::producer_consumer;

#[test]
fn test_collection() {
    let result = producer_consumer();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_length() {
    let result = producer_consumer();
    assert_eq!(result.len(), 5);
}
