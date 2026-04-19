use fundamentals::medium::trait_area::{print_area, Circle, Shape, Square};

#[test]
fn test_circle() {
    let c = Circle { radius: 1.0 };
    assert!((print_area(&c) - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_square() {
    let s = Square { side: 4.0 };
    assert!((print_area(&s) - 16.0).abs() < f64::EPSILON);
}

#[test]
fn test_zero_radius() {
    let c = Circle { radius: 0.0 };
    assert!((print_area(&c) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_large_square() {
    let s = Square { side: 100.0 };
    assert!((print_area(&s) - 10000.0).abs() < f64::EPSILON);
}
