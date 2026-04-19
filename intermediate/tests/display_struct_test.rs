use intermediate::easy::display_struct::Point;

#[test]
fn test_display_basic() {
    let p = Point { x: 1.5, y: 2.0 };
    assert_eq!(format!("{}", p), "(1.50, 2.00)");
}

#[test]
fn test_display_origin() {
    let p = Point { x: 0.0, y: 0.0 };
    assert_eq!(format!("{}", p), "(0.00, 0.00)");
}

#[test]
fn test_display_negative() {
    let p = Point { x: -3.14, y: 2.72 };
    assert_eq!(format!("{}", p), "(-3.14, 2.72)");
}

#[test]
fn test_debug_still_works() {
    let p = Point { x: 1.0, y: 2.0 };
    let debug_str = format!("{:?}", p);
    assert!(debug_str.contains("Point"));
}
