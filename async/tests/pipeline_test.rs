use r#async::medium::pipeline::data_pipeline;

#[test]
fn test_sum() {
    // 1^2 + 2^2 + 3^2 + 4^2 + 5^2 = 1 + 4 + 9 + 16 + 25 = 55
    assert_eq!(data_pipeline(), 55);
}

#[test]
fn test_consistency() {
    for _ in 0..5 {
        assert_eq!(data_pipeline(), 55);
    }
}
