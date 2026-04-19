use fundamentals::easy::tuple_swap::swap_tuple;

#[test]
fn test_swap() {
    assert_eq!(swap_tuple((1, 2)), (2, 1));
}

#[test]
fn test_same_values() {
    assert_eq!(swap_tuple((5, 5)), (5, 5));
}

#[test]
fn test_negative() {
    assert_eq!(swap_tuple((-3, 7)), (7, -3));
}

#[test]
fn test_zero() {
    assert_eq!(swap_tuple((0, 42)), (42, 0));
}
