use fundamentals::easy::sum::add;

#[test]
fn test_positive_numbers() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(add(-1, -1), -2);
}

#[test]
fn test_zero() {
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_mixed_signs() {
    assert_eq!(add(-5, 10), 5);
}
