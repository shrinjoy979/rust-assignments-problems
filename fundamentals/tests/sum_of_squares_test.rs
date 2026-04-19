use fundamentals::easy::sum_of_squares::sum_of_squares;

#[test]
fn test_basic() {
    assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
}

#[test]
fn test_single() {
    assert_eq!(sum_of_squares(&[5]), 25);
}

#[test]
fn test_empty() {
    assert_eq!(sum_of_squares(&[]), 0);
}

#[test]
fn test_negative() {
    assert_eq!(sum_of_squares(&[-3, 4]), 25);
}
