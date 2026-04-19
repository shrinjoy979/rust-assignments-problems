use fundamentals::easy::array_sum::array_sum;

#[test]
fn test_positive() {
    assert_eq!(array_sum(&[1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_zeros() {
    assert_eq!(array_sum(&[0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_negative() {
    assert_eq!(array_sum(&[-1, -2, -3, -4, -5]), -15);
}

#[test]
fn test_mixed() {
    assert_eq!(array_sum(&[10, -10, 20, -20, 0]), 0);
}
