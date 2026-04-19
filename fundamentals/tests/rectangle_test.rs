use fundamentals::easy::rectangle::Rectangle;

#[test]
fn test_area() {
    let r = Rectangle { width: 5.0, height: 3.0 };
    assert!((r.area() - 15.0).abs() < f64::EPSILON);
}

#[test]
fn test_square() {
    let r = Rectangle { width: 4.0, height: 4.0 };
    assert!(r.is_square());
}

#[test]
fn test_not_square() {
    let r = Rectangle { width: 4.0, height: 5.0 };
    assert!(!r.is_square());
}

#[test]
fn test_zero_area() {
    let r = Rectangle { width: 0.0, height: 10.0 };
    assert!((r.area() - 0.0).abs() < f64::EPSILON);
}
