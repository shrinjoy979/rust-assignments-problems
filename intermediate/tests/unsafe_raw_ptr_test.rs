use intermediate::medium::unsafe_raw_ptr::read_via_raw_pointer;

#[test]
fn test_basic() {
    let x = 42;
    assert_eq!(read_via_raw_pointer(&x), 42);
}

#[test]
fn test_negative() {
    let x = -7;
    assert_eq!(read_via_raw_pointer(&x), -7);
}

#[test]
fn test_zero() {
    let x = 0;
    assert_eq!(read_via_raw_pointer(&x), 0);
}
