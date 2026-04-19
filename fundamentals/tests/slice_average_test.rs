use fundamentals::easy::slice_average::average;

#[test]
fn test_basic() {
    assert_eq!(average(&[1.0, 2.0, 3.0]), Some(2.0));
}

#[test]
fn test_single() {
    assert_eq!(average(&[42.0]), Some(42.0));
}

#[test]
fn test_empty() {
    assert_eq!(average(&[]), None);
}

#[test]
fn test_negative() {
    let result = average(&[-2.0, 2.0]).unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}
