use intermediate::medium::enum_dispatch::{Circle, Shape, Square};

#[test]
fn test_circle_area() {
    let s = Shape::Circle(Circle { radius: 1.0 });
    assert!((s.area() - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_square_area() {
    let s = Shape::Square(Square { side: 4.0 });
    assert!((s.area() - 16.0).abs() < f64::EPSILON);
}

#[test]
fn test_large_radius() {
    let s = Shape::Circle(Circle { radius: 10.0 });
    assert!((s.area() - (100.0 * std::f64::consts::PI)).abs() < 1e-8);
}
