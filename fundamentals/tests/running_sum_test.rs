use fundamentals::medium::running_sum::running_sum;

#[test]
fn test_basic() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
}

#[test]
fn test_single() {
    assert_eq!(running_sum(vec![5]), vec![5]);
}

#[test]
fn test_empty() {
    assert_eq!(running_sum(vec![]), vec![]);
}

#[test]
fn test_negative() {
    assert_eq!(running_sum(vec![1, -1, 1, -1]), vec![1, 0, 1, 0]);
}
