use intermediate::easy::newtype_wrapper::{Kilometers, Meters};

#[test]
fn test_conversion() {
    let km = Kilometers::from(Meters(1500.0));
    assert!((km.0 - 1.5).abs() < f64::EPSILON);
}

#[test]
fn test_add_meters() {
    assert_eq!(Meters(100.0) + Meters(200.0), Meters(300.0));
}

#[test]
fn test_zero_conversion() {
    let km = Kilometers::from(Meters(0.0));
    assert!((km.0 - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_into() {
    let km: Kilometers = Meters(5000.0).into();
    assert!((km.0 - 5.0).abs() < f64::EPSILON);
}
