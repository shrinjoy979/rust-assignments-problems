use r#async::easy::threaded_sum::threaded_sum;

#[test]
fn test_even_length() {
    assert_eq!(threaded_sum(vec![1, 2, 3, 4]), 10);
}

#[test]
fn test_odd_length() {
    assert_eq!(threaded_sum(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_empty() {
    assert_eq!(threaded_sum(vec![]), 0);
}

#[test]
fn test_large() {
    let v = vec![1; 1000];
    assert_eq!(threaded_sum(v), 1000);
}
