use fundamentals::medium::generic_find_max::find_max;

#[test]
fn test_ints() {
    assert_eq!(find_max(&[1, 5, 3, 9, 2]), Some(&9));
}

#[test]
fn test_floats() {
    assert_eq!(find_max(&[1.5, 3.7, 2.1]), Some(&3.7));
}

#[test]
fn test_single() {
    assert_eq!(find_max(&[42]), Some(&42));
}

#[test]
fn test_empty() {
    let empty: &[i32] = &[];
    assert_eq!(find_max(empty), None);
}

#[test]
fn test_strings() {
    assert_eq!(find_max(&["apple", "banana", "cherry"]), Some(&"cherry"));
}
