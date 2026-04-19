use intermediate::easy::from_into::{Celsius, Fahrenheit};

#[test]
fn test_freezing() {
    let f: Fahrenheit = Celsius(0.0).into();
    assert!((f.0 - 32.0).abs() < f64::EPSILON);
}

#[test]
fn test_boiling() {
    let f = Fahrenheit::from(Celsius(100.0));
    assert!((f.0 - 212.0).abs() < f64::EPSILON);
}

#[test]
fn test_negative() {
    let f: Fahrenheit = Celsius(-40.0).into();
    assert!((f.0 - (-40.0)).abs() < f64::EPSILON);
}

#[test]
fn test_body_temp() {
    let f = Fahrenheit::from(Celsius(37.0));
    assert!((f.0 - 98.6).abs() < 0.01);
}
