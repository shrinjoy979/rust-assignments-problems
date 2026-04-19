use intermediate::medium::unsafe_swap::unsafe_swap;

#[test]
fn test_swap() {
    let (mut a, mut b) = (5, 10);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 10);
    assert_eq!(b, 5);
}

#[test]
fn test_same_values() {
    let (mut a, mut b) = (7, 7);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 7);
    assert_eq!(b, 7);
}

#[test]
fn test_negative() {
    let (mut a, mut b) = (-1, 1);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 1);
    assert_eq!(b, -1);
}
