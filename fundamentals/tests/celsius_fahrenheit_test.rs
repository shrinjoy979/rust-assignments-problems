use fundamentals::easy::celsius_fahrenheit::celsius_to_fahrenheit;

#[test]
fn test_freezing() {
    assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < f64::EPSILON);
}

#[test]
fn test_boiling() {
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < f64::EPSILON);
}

#[test]
fn test_negative() {
    assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < f64::EPSILON);
}

#[test]
fn test_body_temp() {
    assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 0.01);
}
