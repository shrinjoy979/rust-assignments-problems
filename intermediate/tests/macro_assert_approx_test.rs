#[test]
fn test_equal() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 1.0);
}

#[test]
fn test_close() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
}

#[test]
#[should_panic]
fn test_not_close() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 2.0);
}

#[test]
fn test_custom_epsilon() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 1.05, 0.1);
}
