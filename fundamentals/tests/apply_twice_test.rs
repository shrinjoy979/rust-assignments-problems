use fundamentals::easy::apply_twice::apply_twice;

#[test]
fn test_double() {
    assert_eq!(apply_twice(3, |x| x * 2), 12);
}

#[test]
fn test_increment() {
    assert_eq!(apply_twice(0, |x| x + 1), 2);
}

#[test]
fn test_square() {
    assert_eq!(apply_twice(2, |x| x * x), 16);
}

#[test]
fn test_identity() {
    assert_eq!(apply_twice(5, |x| x), 5);
}
