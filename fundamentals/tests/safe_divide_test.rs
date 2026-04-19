use fundamentals::easy::safe_divide::safe_divide;

#[test]
fn test_normal_division() {
    assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
}

#[test]
fn test_divide_by_zero() {
    assert_eq!(safe_divide(10.0, 0.0), None);
}

#[test]
fn test_zero_dividend() {
    assert_eq!(safe_divide(0.0, 5.0), Some(0.0));
}

#[test]
fn test_negative() {
    assert_eq!(safe_divide(-10.0, 2.0), Some(-5.0));
}
