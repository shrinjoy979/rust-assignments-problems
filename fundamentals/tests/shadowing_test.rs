use fundamentals::easy::shadowing::shadow_transform;

#[test]
fn test_basic() {
    assert_eq!(shadow_transform(5), "10");
}

#[test]
fn test_zero() {
    assert_eq!(shadow_transform(0), "0");
}

#[test]
fn test_large() {
    assert_eq!(shadow_transform(1000), "2000");
}
